#![windows_subsystem = "windows"]
use std::fmt::Display;
use druid::widget::{Button, Flex, Label, RadioGroup, Switch, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use rand::prelude::SliceRandom;
use rand::Rng;

#[derive(Clone, Data, PartialEq)]
enum SortingAlgorithm {
    BubbleSort,
    QuickSort,
}

impl Display for SortingAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SortingAlgorithm::BubbleSort => "BubbleSort",
            SortingAlgorithm::QuickSort => "QuickSort",
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, Data, Lens)]
struct AppState {
    input: String,
    output: String,
    selected_algorithm: SortingAlgorithm,
    input_count: String,
    allow_duplicates: bool,
}

fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let pivot = arr[arr.len() / 2];
    let mut less: Vec<i32> = arr.iter().filter(|&&x| x < pivot).cloned().collect();
    let mut equal: Vec<i32> = arr.iter().filter(|&&x| x == pivot).cloned().collect();
    let mut greater: Vec<i32> = arr.iter().filter(|&&x| x > pivot).cloned().collect();

    less = quick_sort(less);
    greater = quick_sort(greater);

    less.append(&mut equal);
    less.append(&mut greater);
    less
}

fn generate_random_list(size: usize, min: i32, max: i32, allow_duplicates: bool) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    if allow_duplicates {

        (0..size)
            .map(|_| rng.gen_range(min..=max))
            .collect()
    } else {

        let mut numbers: Vec<i32> = (min..=max).collect();
        numbers.shuffle(&mut rng);
        numbers.truncate(size);
        numbers
    }
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("AssSorter")
        .window_size((600.0, 520.0));
    let initial_state = AppState {
        input: "".into(),
        output: "".into(),
        selected_algorithm: SortingAlgorithm::BubbleSort,
        input_count: "18".into(),
        allow_duplicates: true,
    };
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("shit didnt launch, fuck you");
}

fn build_ui() -> impl Widget<AppState> {

    let input = TextBox::new()
        .with_placeholder("enter your numbers, separate by commas")
        .expand_width()
        .lens(AppState::input);

    let output = Label::new(|data: &AppState, _env: &_| format!("{}", data.output))
        .padding(10.0);

    let radio_group = RadioGroup::column(vec![
        ("BubbleSort", SortingAlgorithm::BubbleSort),
        ("QuickSort", SortingAlgorithm::QuickSort),
    ])
        .lens(AppState::selected_algorithm);

    let input_count_label = Label::new("How long do you want this bullshit to be".to_string())
        .padding(5.0);

    let input_count = TextBox::new()
        .with_placeholder("18")
        .fix_width(80.0)
        .fix_height(30.0)
        .lens(AppState::input_count);

    let duplicates_switch = Switch::new()
        .lens(AppState::allow_duplicates);

    let duplicates_label = Label::new("Allow duplicate cases??")
        .padding(5.0);

    let generate_button = Button::new("click if ur lazy")
        .on_click(|_ctx, data: &mut AppState, _env| {

            let count = data.input_count.parse::<usize>().unwrap_or(10);
            let random_list = generate_random_list(count, 1, count as i32, data.allow_duplicates);
            data.input = random_list
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
        })
        .padding(10.0)
        .expand_width();

    let button = Button::new("SORT THIS SHIT OUT")
        .on_click(|_ctx, data: &mut AppState, _env| {
            let nums: Vec<i32> = data
                .input
                .split(',')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();
            let sorted = match data.selected_algorithm {
                SortingAlgorithm::BubbleSort => bubble_sort(nums),
                SortingAlgorithm::QuickSort => quick_sort(nums),
            };
            data.output = sorted.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
        })
        .padding(10.0)
        .expand_width();

    Flex::column()
        .with_child(input)
        .with_spacer(20.0)
        .with_child(radio_group)
        .with_spacer(20.0)
        .with_child(input_count_label)
        .with_spacer(5.0)
        .with_child(input_count)
        .with_spacer(20.0)
        .with_child(duplicates_label)
        .with_spacer(5.0)
        .with_child(duplicates_switch)
        .with_spacer(20.0)
        .with_child(generate_button)
        .with_spacer(10.0)
        .with_child(button)
        .with_spacer(10.0)
        .with_child(output)
        .padding(20.0)
}