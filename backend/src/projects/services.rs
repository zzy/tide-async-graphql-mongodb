use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{oid::ObjectId, Document, doc, to_document, from_document},
};

use crate::util::constant::GqlResult;
use crate::projects::models::{Project, NewProject};

// Create new project
pub async fn add_project(
    db: Database,
    new_project: NewProject,
) -> GqlResult<Project> {
    let coll = db.collection::<Document>("projects");

    let exist_document = coll
        .find_one(
            doc! {"user_id": &new_project.user_id,  "subject": &new_project.subject},
            None,
        )
        .await?;
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let new_project_document = to_document(&new_project)?;

        // Insert into a MongoDB collection
        coll.insert_one(new_project_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let project_document = coll
        .find_one(
            doc! {"user_id": &new_project.user_id,  "subject": &new_project.subject},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let project: Project = from_document(project_document)?;
    Ok(project)
}

// Find all projects
pub async fn all_projects(db: Database) -> GqlResult<Vec<Project>> {
    let coll = db.collection::<Document>("projects");

    let mut projects: Vec<Project> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(projects)
}

// Find all projects by user
pub async fn all_projects_by_user(
    db: Database,
    user_id: ObjectId,
) -> GqlResult<Vec<Project>> {
    let coll = db.collection::<Document>("projects");

    let mut projects: Vec<Project> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(doc! {"user_id": user_id}, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(projects)
}
