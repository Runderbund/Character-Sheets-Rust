use yew::prelude::*;

// This file (lib.rs) defines the library crate for our project.
// It is tied to the cargo.toml file.
// Everything defined here is automatically available to other parts of our project, including main.rs, without needing explicit imports.

// CharacterSheet is a function component that renders the character's information
#[function_component(CharacterSheet)]
pub fn character_sheet() -> Html {
    // Define character stats
    // u8 is an 8-bit unsigned integer, suitable for storing character attributes from 0 to 255
    let health: u8 = 9;
    let stamina: u8 = 4;
    let strength: u8 = 12;
    let mobility: u8 = 7;
    let intelligence: u8 = 2;
    let wisdom: u8 = 1;
    let focus: u8 = 10;

    // The html! macro is used to create the component's HTML structure
    html! {
        // Main container for the character sheet
        <div class="character-sheet">
            // Character main image
            <img src="images//BARBAR.webp" alt="BarBar Smash" class="BarBar"/>
            // Container for character info (level and class)
            <div class="info">
                <h3>{"Level: 2"}</h3>
                <h2>{"Class: Barbellian"}</h2>
            </div>
            // Container for character stats
            <div class="stats">
                // Unordered list to display stats
                <ul>
                    // List items for each stat
                    // format! macro is used to combine the stat name and its value. Like an fstring in python.
                    <li>
                        <span class="stat-name">{"Health"}</span>
                        <span class="stat-value">{health}</span>
                    </li>
                    <li>
                        <span class="stat-name">{"Stamina"}</span>
                        <span class="stat-value">{stamina}</span>
                    </li>
                    <li>
                        <span class="stat-name">{"Strength"}</span>
                        <span class="stat-value">{strength}</span>
                    </li>
                    <li>
                        <span class="stat-name">{"Mobility"}</span>
                        <span class="stat-value">{mobility}</span>
                    </li>
                    <li>
                        <span class="stat-name">{"Intelligence"}</span>
                        <span class="stat-value">{intelligence}</span>
                    </li>
                    <li>
                        <span class="stat-name">{"Wisdom"}</span>
                        <span class="stat-value">{wisdom}</span>
                    </li>
                    <li>
                        <span class="stat-name">{"Focus"}</span>
                        <span class="stat-value">{focus}</span>
                    </li>
                </ul>
            </div>
        </div>
    }
}

