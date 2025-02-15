use color_eyre::Result;

mod menu;
mod constant;

use menu::Menu;

fn main() -> Result<()> {
    let terminal = ratatui::init();

    let result = Menu::default().run(terminal);

    ratatui::restore();
    result
}



// Menu should be
// Implement Menu layout as Widget
// Implement Menu selection as StatefulWidget

// play - human mode - DONE

// train - NEXT - Implement Model Menu With Training, and Storing of weights - DONE
// User can select number of episodes to run for
// When episodes are not done, agent's select action will keep getting called to select an action
// Make it so that user can watch the model select actions
// Now when episode is done, we store the stats in the csv for viewing

// NEXT - Stats Page
// Then on the stats page, you can see the model's stats over time/episodes
// Then for the stats page we display the records kept in the csv. How do we render it? Barcharts? Stat graphs?
//   - model mode
//   - we store the k-distribution of probabilities, then the model's weights then the model's weights progression over training episodes

// stats
//   - we view the k-distribution of probabilities, along with the model weights then the weight progression over time compared to the actual probability distribution