use std::collections::HashMap;
use std::rc::Rc;
use ast::OpCode;
use store::{StoreType, EnumVariant, VariantType};
use store::folder::StoreTypeFolder;
use oil::parse_expression;
use ast::ExprWithIdEqZero;
use ast::path::into_linpath;
use super::store_folder::CompileFolder;
use super::render::Render;
use super::{AssignIR, ExprIR, VarIR, IR};



#[test]
fn rendering_of_folder() {
    let mut store = make_store();
    let mut folder = CompileFolder;

    let ir = folder.fold(&store, &into_linpath(&parse_expression(&ExprWithIdEqZero,
        "store.a.b?c?"
    ).unwrap().unwrap_path()).unwrap(), 0).expect("No IR");
    let mut output = String::new();
    ir.render(&mut output);
    println!("{}", output);
//        assert_eq!(true, false);
}

#[test]
fn rendering_test() {
    let mut store = make_store();
    let mut folder = CompileFolder;

    let ir = folder.fold(&store, &into_linpath(&parse_expression(&ExprWithIdEqZero,
        "store.a.b?c?"
    ).unwrap().unwrap_path()).unwrap(), 0).expect("No IR");

    let first_assign = Rc::new(AssignIR {
        leftop: VarIR { id: 0 },
        rightop: ExprIR::Path(ir),
    });
    let ir = vec![first_assign.clone(), Rc::new(AssignIR {
        leftop: VarIR { id: 1 },
        rightop: ExprIR::BinaryOp(first_assign.clone(), OpCode::Add, first_assign.clone()),
    })];
    let mut output = String::new();
    IR { instructions: ir}.render(&mut output);
    println!("{}", output);
    assert_eq!(true, false);
}


fn make_store() -> StoreType {
    StoreType::Struct { fields: {
        let mut h = HashMap::new();
        h.insert("a".to_string(), StoreType::Enum {
            variants: vec![
                EnumVariant {
                    name: "A1".into(),
                    variant_type: VariantType::Struct {
                        fields: {
                            let mut h = HashMap::new();
                            h.insert("b".into(), StoreType::Enum {
                                variants: vec![
                                    EnumVariant {
                                        name: "B2".into(),
                                        variant_type: VariantType::Struct {
                                            fields: {
                                                let mut h = HashMap::new();
                                                h.insert("c".into(), StoreType::Number);
                                                h
                                            },
                                        },
                                    },
                                ],
                            });
                            h.insert("e".into(), StoreType::Number);
                            h
                        },
                    },
                },
                EnumVariant {
                    name: "A2".into(),
                    variant_type: VariantType::Struct {
                        fields: {
                            let mut h = HashMap::new();
                            h.insert("b".into(), StoreType::Struct {
                                fields: {
                                    let mut h = HashMap::new();
                                    h.insert("c".into(), StoreType::String);
                                    h
                                },
                            });
                            h
                        },
                    },
                },
                EnumVariant {
                    name: "A3".into(),
                    variant_type: VariantType::Struct {
                        fields: {
                            let mut h = HashMap::new();
                            h.insert("b".into(), StoreType::Enum {
                                variants: vec![
                                    EnumVariant {
                                        name: "V1".into(),
                                        variant_type: VariantType::Tuple { elements: vec![StoreType::Number] },
                                    },
                                    EnumVariant {
                                        name: "V2".into(),
                                        variant_type: VariantType::Tuple { elements: vec![StoreType::String] },
                                    },
                                    EnumVariant {
                                        name: "V3".into(),
                                        variant_type: VariantType::Struct {
                                            fields: {
                                                let mut h = HashMap::new();
                                                h.insert("h".into(), StoreType::String);
                                                h
                                            },
                                        },
                                    },
                                ],
                            });
                            h
                        },
                    },
                },
            ]
        });
        h
    }}
}