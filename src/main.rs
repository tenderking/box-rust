use std::collections::HashMap;

fn target_height(boxes: &HashMap<&str, f32>) -> f32 {
    let box_array = boxes.values().collect::<Vec<&f32>>();
    println!("{:?}", box_array);
    let target_height: f32 = box_array.iter().copied().sum::<f32>() / 3.0;
    return target_height;
}
//
fn combinations<T: Clone>(list: &Vec<T>, length: usize) -> Vec<Vec<T>> {
    if length == 0 {
        return vec![vec![]];
    }

    let mut result: Vec<Vec<T>> = Vec::new();
    for i in 0..list.len() {
        let current_element = list[i].clone(); // Clone the element
        let sub_combinations = combinations(&list[i + 1..].to_vec(), length - 1);
        for sub_combination in sub_combinations {
            let mut new_combination = sub_combination;
            new_combination.push(current_element.clone()); // Clone the element again
            result.push(new_combination);
        }
    }

    result
}

fn find_solutions(boxes: &Vec<f32>, target_height: f32) -> Vec<Vec<f32>> {
    let three_boxes_list = combinations(boxes, 3);
    let four_boxes_list = combinations(boxes, 4);
    let mut solutions = vec![];
    let threshold = (target_height - 0.3, target_height + 0.3);
    for box_combo in three_boxes_list.iter().chain(four_boxes_list.iter()) {
        let box_combo_output: f32 = box_combo.iter().collect::<Vec<_>>().into_iter().sum();
        if box_combo_output > threshold.0 && box_combo_output < threshold.1 {
            solutions.push(box_combo.to_vec());
        }
    }
    return solutions;
}


fn filter_include_all_boxes(solutions: Vec<Vec<f32>>, boxes: HashMap<&str, f32>) -> Vec<Vec<Vec<f32>>> {
    let mut result: Vec<Vec<Vec<f32>>> = Vec::new();
    let box_array: Vec<_> = boxes.values().copied().collect();
    for vec in &solutions {
        for vec2 in &solutions {
            if vec.iter().all(|&value| !vec2.contains(&value)) {
                let remaining_values: Vec<f32> = box_array
                    .iter()
                    .filter(|&value| !vec.contains(value) && !vec2.contains(value))
                    .copied()
                    .collect();
                let mut combined = vec![vec.clone(), vec2.clone(), remaining_values];

                // Sort each inner Vec<f32>
                for inner_vec in &mut combined {
                    inner_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
                }

                // Sort the combined Vec<Vec<f32>> (optional)
                combined.sort_by(|a, b| a.partial_cmp(b).unwrap());
                result.push(combined);
            }
        }
    }
    return result;
}

fn find_best_solution(solutions: &Vec<Vec<Vec<f32>>>) ->(usize, f32)  {
    let answer = solutions
        .into_iter()
        .map(|inner| inner.into_iter().map(|vec| vec.iter().sum::<f32>()).collect())
        .collect();
    return find_least_height_diff(answer)
}

fn find_least_height_diff(solutions: Vec<Vec<f32>>) -> (usize, f32) {
    let mut min_diff = f32::INFINITY;
    let mut min_diff_index = 0;

    for (i, solution) in solutions.iter().enumerate() {
        let max = solution.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min = solution.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let diff = max - min;

        if diff < min_diff {
            min_diff = diff;
            min_diff_index = i;
        }
    }

    (min_diff_index, min_diff)
}

fn map_values_to_keys(
    solutions: &Vec<Vec<f32>>,
    box_values: &HashMap<&str, f32>,
) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let box_values_vec: Vec<(&str, f32)> = box_values.iter().map(|(k, &v)| (*k, v)).collect();

    for solution in solutions {
        let mut keys: Vec<String> = Vec::new();
        for &value in solution {
            for &(key, box_value) in &box_values_vec {
                if value == box_value {
                    keys.push(key.to_string());
                    break;
                }
            }
        }
        result.push(keys);
    }

    result
}

fn main() {
    let box_values = HashMap::from([
        ("box1", 4.0),
        ("box2", 4.9),
        ("box3", 6.0),
        ("box4", 7.2),
        ("box5", 8.3),
        ("box6", 9.5),
        ("box7", 10.5),
        ("box8", 11.3),
        ("box9", 12.5),
        ("box10", 13.6),
    ]);

    let box_array = &box_values.values().copied().collect::<Vec<f32>>();
    println!("target value is {:?}", target_height(&box_values));
    let box_combination = combinations(&box_array, 3);
    println!("box combinations {:?}", box_combination.len());
    let solutions = find_solutions(&box_array, target_height(&box_values));
    let valid_solutions = filter_include_all_boxes(solutions, box_values.clone());
    println!("valid solutions: {:?}",&valid_solutions.len());
    let (index, size)  = find_best_solution(&valid_solutions);
    let answer = &valid_solutions[index];
    println!("answer is {:?} with error of {:?}", map_values_to_keys(answer, &box_values),size);

}
