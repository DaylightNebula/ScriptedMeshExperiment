use glam::*;
use rhai::*;

#[derive(Clone)]
pub struct ScriptedMesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub normals: Vec<Vec3>
}

impl ScriptedMesh {
    fn new() -> Self {
        Self { positions: Vec::new(), indices: Vec::new(), normals: Vec::new() }
    }
}

impl CustomType for ScriptedMesh {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Mesh")
            .with_fn("new_mesh", Self::new);
    }
}

pub struct ScriptedMeshEngine {
    engine: Engine
}

#[derive(Debug)]
pub enum ScriptedMeshLoadError {
    FileReadError(std::io::Error),
    CompileError(ParseError),
    EvalError(Box<EvalAltResult>)
}

impl ScriptedMeshEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        engine.build_type::<ScriptedMesh>();
        Self { engine } 
    }

    pub fn from_path(&self, path: impl Into<String>) -> Result<ScriptedMesh, ScriptedMeshLoadError> {
        // get text from the given path
        let text = std::fs::read_to_string(path.into());
        let text = if text.is_ok() { text.unwrap() } else { return Err(ScriptedMeshLoadError::FileReadError(text.err().unwrap())) };

        // return the result from the text
        return self.from_text(text);
    }

    pub fn from_text(&self, text: impl Into<String>) -> Result<ScriptedMesh, ScriptedMeshLoadError> {
        // compile given text into a script (ast)
        let ast = self.engine.compile(text.into());
        let ast = if ast.is_ok() { ast.unwrap() } else { return Err(ScriptedMeshLoadError::CompileError(ast.err().unwrap())); };

        // run the script
        let result: Result<ScriptedMesh, Box<EvalAltResult>> = self.engine.eval_ast::<ScriptedMesh>(&ast);
        let result = if result.is_ok() { result.unwrap() } else { return Err(ScriptedMeshLoadError::EvalError(result.err().unwrap())); };

        // pass back the result
        return Ok(result);
    }
}