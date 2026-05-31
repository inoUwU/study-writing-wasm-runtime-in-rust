#[derive(Debug, PartialEq, Eq)]
pub struct Module  {
    pub magic:String,
    pub version: u32,
}

#[cfg([test])]
mod tests{
    use crate::binary::module:Module;
    use anyhow::Result;

    #[test]
    fn decode_simplest_module() -> Result<()>{
        // プリアンプルしか存在しないwasmバイナリを生成
        let wasm = wat::
    }

}
