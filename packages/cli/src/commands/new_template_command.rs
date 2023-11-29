pub struct NewTemplateCommand;

impl NewTemplateCommand {
    pub fn new() -> NewTemplateCommand {
        NewTemplateCommand {}
    }

    pub fn execute(&self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        let template_name = args.get(0).unwrap();
        let template_description = args.get(1).unwrap();
        let template_path = args.get(2).unwrap();

        let template = Template::new();
        let template = template
            .with_name(template_name)
            .with_description(template_description)
            .with_path(template_path);

        let json = template.to_json()?;
        println!("{}", json);

        Ok(())
    }
}
