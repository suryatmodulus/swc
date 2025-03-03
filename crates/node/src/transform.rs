use crate::{
    get_compiler,
    util::{deserialize_json, get_deserialized, try_with, MapErr},
};
use anyhow::Context as _;
use napi::{
    bindgen_prelude::{AbortSignal, AsyncTask, Buffer},
    Env, Task,
};
use path_clean::clean;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use swc::{config::Options, Compiler, TransformOutput};
use swc_common::FileName;
use swc_ecma_ast::Program;

/// Input to transform
#[derive(Debug)]
pub enum Input {
    /// json string
    Program(String),
    /// Raw source code.
    Source { src: String },
    /// File
    File(PathBuf),
}

pub struct TransformTask {
    pub c: Arc<Compiler>,
    pub input: Input,
    pub options: String,
}

#[napi]
impl Task for TransformTask {
    type Output = TransformOutput;
    type JsValue = TransformOutput;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        let mut options: Options = deserialize_json(&self.options)?;
        if !options.filename.is_empty() {
            options.config.adjust(Path::new(&options.filename));
        }

        try_with(
            self.c.cm.clone(),
            !options.config.error.filename,
            |handler| {
                self.c.run(|| match &self.input {
                    Input::Program(ref s) => {
                        let program: Program =
                            deserialize_json(s).expect("failed to deserialize Program");
                        // TODO: Source map
                        self.c.process_js(handler, program, &options)
                    }

                    Input::File(ref path) => {
                        let fm = self.c.cm.load_file(path).context("failed to load file")?;
                        self.c.process_js_file(fm, handler, &options)
                    }

                    Input::Source { src } => {
                        let fm = self.c.cm.new_source_file(
                            if options.filename.is_empty() {
                                FileName::Anon
                            } else {
                                FileName::Real(options.filename.clone().into())
                            },
                            src.to_string(),
                        );

                        self.c.process_js_file(fm, handler, &options)
                    }
                })
            },
        )
        .convert_err()
    }

    fn resolve(&mut self, _env: Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(result)
    }
}

#[napi]
pub fn transform(
    src: String,
    is_module: bool,
    options: Buffer,
    signal: Option<AbortSignal>,
) -> napi::Result<AsyncTask<TransformTask>> {
    let c = get_compiler();

    let options = String::from_utf8_lossy(options.as_ref()).to_string();

    let input = if is_module {
        Input::Program(src)
    } else {
        Input::Source { src }
    };

    let task = TransformTask { c, input, options };
    Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn transform_sync(s: String, is_module: bool, opts: Buffer) -> napi::Result<TransformOutput> {
    let c = get_compiler();

    let mut options: Options = get_deserialized(&opts)?;

    if !options.filename.is_empty() {
        options.config.adjust(Path::new(&options.filename));
    }

    try_with(c.cm.clone(), !options.config.error.filename, |handler| {
        c.run(|| {
            if is_module {
                let program: Program =
                    deserialize_json(s.as_str()).context("failed to deserialize Program")?;
                c.process_js(handler, program, &options)
            } else {
                let fm = c.cm.new_source_file(
                    if options.filename.is_empty() {
                        FileName::Anon
                    } else {
                        FileName::Real(options.filename.clone().into())
                    },
                    s,
                );
                c.process_js_file(fm, handler, &options)
            }
        })
    })
    .convert_err()
}

#[napi]
pub fn transform_file(
    src: String,
    _is_module: bool,
    options: Buffer,
    signal: Option<AbortSignal>,
) -> napi::Result<AsyncTask<TransformTask>> {
    let c = get_compiler();

    let options = String::from_utf8_lossy(options.as_ref()).to_string();
    let path = clean(&src);
    let task = TransformTask {
        c,
        input: Input::File(path.into()),
        options,
    };
    Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn transform_file_sync(
    s: String,
    is_module: bool,
    opts: Buffer,
) -> napi::Result<TransformOutput> {
    let c = get_compiler();

    let mut options: Options = get_deserialized(&opts)?;

    if !options.filename.is_empty() {
        options.config.adjust(Path::new(&options.filename));
    }

    try_with(c.cm.clone(), !options.config.error.filename, |handler| {
        c.run(|| {
            if is_module {
                let program: Program =
                    deserialize_json(s.as_str()).context("failed to deserialize Program")?;
                c.process_js(handler, program, &options)
            } else {
                let fm = c.cm.load_file(Path::new(&s)).expect("failed to load file");
                c.process_js_file(fm, handler, &options)
            }
        })
    })
    .convert_err()
}
