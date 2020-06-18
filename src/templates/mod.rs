use handlebars::Handlebars;
use serde::Serialize;
use lazy_static::lazy_static;


lazy_static! {
  pub static ref TEMPLATE_ENGINE: Handlebars<'static> = {
    let mut hb = Handlebars::new();
    hb.register_template_file("query", "./src/templates/query.hbs")
      .unwrap();
    hb.register_template_file("template", "./src/templates/html.hbs")
      .unwrap();
    println!("Creating template engine!");
    hb
  };
}

pub struct WithTemplate<T: Serialize> {
  pub name: &'static str,
  pub value: T,
}

/**
 * Generic template rendering using WithTemplate struct
 */
pub fn render<T>(
  template: WithTemplate<T>,
  //hbs: Arc<Handlebars>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: Serialize,
{
  let render = TEMPLATE_ENGINE
    .render(template.name, &template.value)
    .unwrap_or_else(|err| err.to_string());
  Ok(warp::reply::html(render))
}