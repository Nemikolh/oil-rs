use super::oil::{parse_ui_package};


#[test]
fn test_mixed_style_template() {
    let view = r#"
    // Here material-oil is one file.
    import {progress-bar} from 'material-oil';
    import {logo} from 'material-oil';

    // This class can't be used elsewhere
    // It is local to the file
    .local-class {
      background-color: #FFFFFF;
    }

    // This template is local to this class
    template local-btn = <button class="local-class"><select:children /></button>

    // ui-element is exported and can be used elsewhere
    export template ui-element [name, progress] =
      <local-btn>Hello {name}</local-btn>
      <d>
        <progress-bar value={progress}></progress-bar>
        <logo></logo>
      </d>
    "#;
    parse_ui_package(view).unwrap();
}
