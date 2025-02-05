use color_eyre::Result;

mod menu;
mod constant;

use menu::Menu;

fn main() -> Result<()> {
    let terminal = ratatui::init();

    let result = Menu::default().run(terminal);

    ratatui::restore();
    result

    // Build terminal ui

    // build game,

    // Game ui should show action estimates on the left side

    // Press key to start

    // On start, action probabilities are set for the k-actions, they can't be the same, for k<5

    // Then each "round" represents an action press, then reward reception

    // Could be multiple rounds

    // Game is who gets to 30 points in the least possible time

    // Create menu option for model to play

    // Model plays and updates action estimates based on cumulative average of rewards

    // As model keeps on playing the assumption is its estimate  keeps improving
}



// Menu should be
// Implement Menu layout as Widget
// Implement Menu selection as StatefulWidget

// play - human mode

// train
//   - model mode
//   - we store the k-distribution of probabilities, then the model's weights then the model's weights progression over training episodes

// stats
//   - we view the k-distribution of probabilities, along with the model weights then the weight progression over time compared to the actual probability distribution