    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum Skill {
        Python,
        JavaScript,
        Rust,
        Go,
        Java,
        Cpp,
        CSharp,
        Swift,
        Kotlin,
        TypeScript,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SkillTree {
        pub skills: HashMap<Skill, u32>,
    }

    impl SkillTree {
        pub fn new() -> Self {
            let mut skills = HashMap::new();
            skills.insert(Skill::Python, 0);
            skills.insert(Skill::JavaScript, 0);
            skills.insert(Skill::Rust, 0);
            // ... initialize all skills to level 0
            SkillTree { skills }
        }

        pub fn upgrade_skill(&mut self, skill: Skill) -> Result<(), &'static str> {
            let level = self.skills.entry(skill).or_insert(0);
            *level += 1;
            Ok(())
        }
        
        pub fn display(&self) {
            println!("  --- Skills ---");
            for (skill, level) in &self.skills {
                if *level > 0 {
                    println!("    - {:?}: Level {}", skill, level);
                }
            }
        }
    }
