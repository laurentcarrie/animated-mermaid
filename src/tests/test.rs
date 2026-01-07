#[cfg(test)]
mod test {
    use super::super::super::mermaid_of_frame::mermaid_of_frame;
    use super::super::super::mermaid_of_frame::tags_of_frame;
    use super::super::super::process_diagram::extract_meta;
    use std::collections::HashMap;

    fn data() -> String {
        return String::from_utf8(include_bytes!("flowchart.html").to_vec()).unwrap();
    }

    #[test]
    fn test_frame_1() {
        let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("info"));
        let data = data();
        let (meta, mermaid) = extract_meta(&data).unwrap();
        let meta_animate = meta.animate.clone().unwrap();
        let frames = meta.animate.unwrap().frames;
        let frame = frames.get(0).unwrap();
        log::info!("frame: {:?}", &frame);
        log::info!("mermaid: {}", &mermaid);
        assert_eq!(
            frame.title,
            "<p style=\"background-color:aquamarine\">Frame 1</p>\n"
        );
        let new_mermaid = mermaid_of_frame(mermaid, &meta_animate, 0).unwrap();
        let expected = String::from_utf8(include_bytes!("flowchart-1.html").to_vec()).unwrap();
        assert_eq!(new_mermaid.trim(), expected.trim());
    }

    #[test]
    fn test_frame_2() {
        let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("info"));
        let data = data();
        let (meta, mermaid) = extract_meta(&data).unwrap();
        let meta_animate = meta.animate.clone().unwrap();
        let frames = meta.animate.unwrap().frames;
        let frame = frames.get(1).unwrap();
        log::info!("frame: {:?}", &frame);
        log::info!("mermaid: {}", &mermaid);
        assert_eq!(
            frame.title,
            "<p style=\"background-color:coral\">Frame 2</p>\n"
        );
        let new_mermaid = mermaid_of_frame(mermaid, &meta_animate, 1).unwrap();
        let expected = String::from_utf8(include_bytes!("flowchart-2.html").to_vec()).unwrap();
        assert_eq!(new_mermaid.trim(), expected.trim());
    }

    #[test]
    fn test_history() {
        let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("info"));
        let data = data();
        let (meta, _mermaid) = extract_meta(&data).unwrap();
        let history = tags_of_frame(&meta.animate.unwrap());
        let expected = vec![
            {
                let mut map = HashMap::new();
                map.insert("Atag".to_string(), "class A red ;".to_string());
                map.insert("B".to_string(), "class B blue ;".to_string());
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("Atag".to_string(), "class A red ;".to_string());
                map.insert("B".to_string(), "class B white ;".to_string());
                map
            },
        ];
        log::info!("history: {:?}", &history);
        log::info!("expected: {:?}", &expected);
        for i in 0..1 {
            for k in vec!["Atag", "B"] {
                assert_eq!(
                    history.get(i).unwrap().get(k).unwrap(),
                    expected.get(i).unwrap().get(k).unwrap()
                );
            }
        }
    }

    #[test]
    fn test_extra_meta() {
        let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("info"));
        let data = data();
        let (meta, mermaid) = extract_meta(&data).unwrap();
        assert!(meta.config.is_some());
        let config = meta.config.unwrap();
        log::info!("config: {:?}", &config);
        let flowchart = config.get("flowchart").unwrap();
        let curve = flowchart.get("curve").unwrap().as_str().unwrap();
        assert_eq!(curve, "basis");
    }
}
