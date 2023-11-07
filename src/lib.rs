use glam::*;
use rhai::*;

#[derive(Debug, Clone)]
pub struct ScriptedMesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub normals: Vec<Vec3>
}

impl ScriptedMesh {
    pub fn new() -> Self {
        Self { 
            positions: Vec::new(), 
            indices: Vec::new(), 
            normals: Vec::new()
        }
    }

    pub fn vec(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }

    // insert and remove functions
    pub fn insert_position(&mut self, position: Vec3) { self.positions.push(position); }
    pub fn insert_index(&mut self, index: i64) { self.indices.push(index as u32); }
    pub fn insert_index_set(&mut self, a: i64, b: i64, c: i64) { self.indices.push(a as u32); self.indices.push(b as u32); self.indices.push(c as u32); }
    pub fn insert_normal(&mut self, normals: Vec3) { self.normals.push(normals); }

    pub fn insert_positions(&mut self, new: Array) { self.positions.append(&mut array_to_vec(new)); }
    pub fn insert_indices(&mut self, new: Array) { self.indices.append(&mut array_to_vec_u32(new)); }
    pub fn insert_normals(&mut self, new: Array) { self.normals.append(&mut array_to_vec(new)); }

    pub fn clear_positions(&mut self) { self.positions.clear(); }
    pub fn clear_indices(&mut self) { self.indices.clear(); }
    pub fn clear_normals(&mut self) { self.normals.clear(); }

    pub fn remove_positions(&mut self, index: i64) -> Vec3 { self.positions.remove(index as usize) }
    pub fn remove_index(&mut self, index: i64) -> u32 { self.indices.remove(index as usize) }
    pub fn remove_normal(&mut self, index: i64) -> Vec3 { self.normals.remove(index as usize) }

    // getters and setters
    pub fn get_positions(&mut self) -> Vec<Vec3> { self.positions.clone() }
    pub fn get_indices(&mut self) -> Vec<u32> { self.indices.clone() }
    pub fn get_normals(&mut self) -> Vec<Vec3> { self.normals.clone() }
    pub fn set_positions(&mut self, new: Vec<Vec3>) { self.positions = new; }
    pub fn set_indices(&mut self, new: Vec<u32>) { self.indices = new; }
    pub fn set_normals(&mut self, new: Vec<Vec3>) { self.normals = new; }

    // debug utils
    pub fn to_string(&mut self) -> String { String::new() }
    pub fn to_debug(&mut self) -> String { format!("{:?}", self) }
}

impl CustomType for ScriptedMesh {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Mesh")
            .on_print(Self::to_string)
            .on_debug(Self::to_debug)
            .with_fn("new_mesh", Self::new)
            .with_fn("vec", Self::vec)
            .with_fn("insert_position", Self::insert_position)
            .with_fn("insert_index", Self::insert_index)
            .with_fn("insert_index_set", Self::insert_index_set)
            .with_fn("insert_normal", Self::insert_normal)
            .with_fn("insert_positions", Self::insert_positions)
            .with_fn("insert_indices", Self::insert_indices)
            .with_fn("insert_normals", Self::insert_normals)
            .with_fn("clear_positions", Self::clear_positions)
            .with_fn("clear_indices", Self::clear_indices)
            .with_fn("clear_normals", Self::clear_normals)
            .with_fn("remove_positions", Self::remove_positions)
            .with_fn("remove_index", Self::remove_index)
            .with_fn("remove_normal", Self::remove_normal)
            .with_get_set("positions", Self::get_positions, Self::set_positions)
            .with_get_set("indices", Self::get_indices, Self::set_indices)
            .with_get_set("normals", Self::get_normals, Self::set_normals);
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

pub fn array_to_vec<T: Clone + 'static>(input: Array) -> Vec<T> {
    input.iter().map(|a| a.clone().cast::<T>()).collect::<Vec<T>>()
}

pub fn array_to_vec_u32(input: Array) -> Vec<u32> {
    input.iter().map(|a| a.as_int().unwrap() as u32).collect::<Vec<u32>>()
}
