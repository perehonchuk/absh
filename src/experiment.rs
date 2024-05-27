use crate::ansi;
use crate::ansi::AnsiColor;
use crate::bars::PlotHighlight;
use crate::experiment_name::ExperimentName;
use crate::math::numbers::Numbers;
use crate::measure::map::MeasureMap;

pub struct Experiment {
    pub name: ExperimentName,
    pub warmup: String,
    pub run: String,
    pub measures: MeasureMap<Numbers>,
}

impl Experiment {
    pub fn plot_highlights(&self) -> PlotHighlight {
        PlotHighlight {
            non_zero: format!("{}", self.name.color().to_owned()),
            zero: format!("{}", AnsiColor::White.bg()),
            reset: ansi::RESET.to_owned(),
        }
    }

    pub fn plot_halves_highlights(&self) -> PlotHighlight {
        PlotHighlight {
            non_zero: format!("{}", self.name.color().to_owned()),
            zero: "".to_owned(),
            reset: ansi::RESET.to_owned(),
        }
    }

    pub fn runs(&self) -> usize {
        self.measures.values().next().unwrap().len()
    }
}
