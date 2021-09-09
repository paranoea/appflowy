use flowy_test::builder::DocTest;

use flowy_document::{entities::doc::*, event::EditorEvent::*};
use flowy_infra::uuid;
use flowy_test::prelude::*;

pub fn create_doc(sdk: &FlowyTestSDK, name: &str, desc: &str, text: &str) -> Doc {
    let request = CreateDocRequest {
        id: uuid(),
        name: name.to_owned(),
        desc: desc.to_owned(),
        data: text.to_owned(),
    };

    let doc = DocTest::new(sdk.clone())
        .event(CreateDoc)
        .request(request)
        .sync_send()
        .parse::<Doc>();
    doc
}

pub fn save_doc(sdk: &FlowyTestSDK, desc: &Doc, content: &str) {
    let request = UpdateDocRequest {
        id: desc.id.clone(),
        name: Some(desc.name.clone()),
        desc: Some(desc.desc.clone()),
        data: Some(content.to_owned()),
    };

    let _ = DocTest::new(sdk.clone()).event(UpdateDoc).request(request).sync_send();
}

// #[allow(dead_code)]
// pub fn read_doc(doc_id: &str) -> DocInfo {
//     let request = QueryDocRequest {
//         doc_id: doc_id.to_string(),
//     };
//
//     let doc = AnnieTestBuilder::new()
//         .event(ReadDocInfo)
//         .request(request)
//         .sync_send()
//         .parse::<DocInfo>();
//
//     doc
// }

pub(crate) fn read_doc_data(sdk: &FlowyTestSDK, doc_id: &str, path: &str) -> DocData {
    let request = QueryDocDataRequest {
        doc_id: doc_id.to_string(),
        path: path.to_string(),
    };

    let doc = DocTest::new(sdk.clone())
        .event(ReadDocData)
        .request(request)
        .sync_send()
        .parse::<DocData>();

    doc
}
