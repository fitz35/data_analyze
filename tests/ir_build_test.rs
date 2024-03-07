use plot_helper::static_html::presentation_data::{Array, Content, Element, Ir, ListElement};



const IR_VALID_PATH: &'static str = "tests/ressources/presentation/testIrValid";
const IR_INVALID_PATH: &'static str = "tests/ressources/presentation/testIrInvalid";

fn get_ref_ir() -> Ir {
    let list : ListElement = vec![
        Element::new(
            "test1".to_string(),
            vec![
                Content::Array(
                    Array::new(
                        vec!["a".to_string(), "b".to_string(), "c".to_string()],
                        vec![
                            vec!["1".to_string(), "2".to_string(), "3".to_string()]
                        ]
                    )
                ).into(),
                Content::Image(format!("{}/test1/img3.png", IR_VALID_PATH)).into(),
                Element::new(
                    "test1.1".to_string(),
                    vec![
                        Content::Image(format!("{}/test1/test1.1/img1.png", IR_VALID_PATH)).into(),
                    ]
                ).into(),
            ]
        ),
        Element::new(
            "test2".to_string(),
            vec![
                Content::Image(format!("{}/test2/img2.png", IR_VALID_PATH)).into(),
            ]
        ),
    ].into();

    list.into()
}


#[test]
fn ir_build_valid_test(){
    let ir = Ir::new_from_file_system(IR_VALID_PATH);
    assert!(ir.is_ok());
    let ir = ir.unwrap();
    assert_eq!(ir, get_ref_ir());
    let html = ir.to_html();
    assert!(html.is_ok());
}

#[test]
fn ir_build_invalid_test(){
    let ir = Ir::new_from_file_system(IR_INVALID_PATH);
    assert!(ir.is_err());
}