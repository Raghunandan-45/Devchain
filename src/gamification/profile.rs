use serde::{Serialize, Deserialize};
use crate::gamification::skills::{Skill, SkillTree};
use crate::gamification::badges::{Badge};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeveloperProfile {
    pub developer_id: String,
    pub xp: u32,
    pub level: u32,
    pub xp_to_next_level: u32,
    pub skill_points: u32,
    pub skill_tree: SkillTree,
    pub badges: HashSet<Badge>,
    pub solved_languages: HashSet<String>,
    pub guild_id: Option<String>,
}

impl DeveloperProfile {
    pub fn new(developer_id: &str) -> Self {
        DeveloperProfile {
            developer_id: developer_id.to_string(),
            xp: 0,
            level: 1,
            xp_to_next_level: 100,
            skill_points: 0,
            skill_tree: SkillTree::new(),
            badges: HashSet::new(),
            solved_languages: HashSet::new(),
            guild_id: None,
        }
    }

    pub fn add_xp(&mut self, amount: u32, language: &str) {
        self.xp += amount;
        println!("\n[+] {} earned {} XP for solving a {} challenge!", self.developer_id, amount, language);
        
        self.solved_languages.insert(language.to_string());
        self.check_for_new_badges();

        while self.xp >= self.xp_to_next_level {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.xp -= self.xp_to_next_level;
        self.level += 1;
        self.skill_points += 1;
        self.xp_to_next_level = (self.xp_to_next_level as f32 * 1.5) as u32;
        println!("[*] DING! {} reached Level {}!", self.developer_id, self.level);
        println!("[*] You have earned 1 Skill Point! You now have {}.", self.skill_points);
    }
    
    fn check_for_new_badges(&mut self) {
        if self.solved_languages.len() >= 3 && !self.badges.contains(&Badge::Polyglot) {
            self.badges.insert(Badge::Polyglot);
            println!("[ACHIEVEMENT] Unlocked: Polyglot (Solved challenges in 3+ languages)");
        }
    }

    pub fn display(&self) {
        println!("\n--- Developer Profile: {} ---", self.developer_id);
        println!("  Level: {} ({} / {} XP)", self.level, self.xp, self.xp_to_next_level);
        println!("  Skill Points: {}", self.skill_points);
        println!("  Guild: {}", self.guild_id.as_deref().unwrap_or("None"));
        println!("  Badges: {:?}", self.badges);
        self.skill_tree.display();
        println!("------------------------------------");
    }
}
