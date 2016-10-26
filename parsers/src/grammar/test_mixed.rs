use super::parse_grammar;


#[test]
fn test_mixed_style_component() {
    let view = r#"
    // Here material-oil is one file.
    import {progress_bar} from 'material-oil';
    import {logo} from 'material-oil';

    // This class can't be used elsewhere
    // It is local to the file
    .local_class {
      background_color: #FFFFFF;
    }

    // This component is local to this class
    component local_btn = <button class=local_class><select:children /></button>;

    // ui_element is exported and can be used elsewhere
    export component ui_element [name, progress] =
      <group>
        <local_btn>Hello {{name}}</local_btn>
        <d>
          <progress_bar [value]={progress}></progress_bar>
          <logo></logo>
        </d>
      </group>;
    "#;
    parse_grammar(view).unwrap();
}
