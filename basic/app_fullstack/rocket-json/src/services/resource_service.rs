use crate::models::resource::Resource;
use crate::dtos::resource_dto::ResourceDTO;

pub fn resource_list() -> Vec<Resource> {
    return vec![
        Resource{ id: 1, title: "Resource 1".to_string(), description: "Description resource 1".to_string() },
        Resource{ id: 2, title: "Resource 2".to_string(), description: "Description resource 2".to_string() },
    ];
}

pub fn create_resource(resource_dto: ResourceDTO) -> Result<Resource, String> {
  println!("Title: {}", resource_dto.title);
  println!("Description: {}", resource_dto.description);

  if true {
      Ok(Resource { id: 3, title: resource_dto.title, description: resource_dto.description })
  } else {
      Err("Failed to create resource".to_string())
  }
}

pub fn update_resource(id: i32, resource_dto: ResourceDTO) -> Result<Resource, String> {
  println!("ID: {}", id);
  println!("Title: {}", resource_dto.title);
  println!("Description: {}", resource_dto.description);

  if true {
      Ok(Resource { id, title: resource_dto.title, description: resource_dto.description })
  } else {
      Err("Failed to update resource".to_string())
  }
}

pub fn get_by_id(id: i32) -> Resource {
    // Simulate fetching a resource by ID from a database or other data source
    println!("{}", id);

    Resource{ id: 1, title: "Resource 1".to_string(), description: "Description resource 1".to_string() }
}

pub fn delete_resource(id: i32) -> Result<(), String> {
    // Simulate deleting a resource by ID from a database or other data source
    println!("ID: {}", id);

    if true {
        Ok(())
    } else {
        Err("Failed to delete resource".to_string())
    }
}
