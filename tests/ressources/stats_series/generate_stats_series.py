import argparse
from dataclasses import dataclass, asdict
import json
import os
from faker import Faker
import numpy as np



def generate_random_float_array(fake : Faker, size: int) -> list[float]:
    return [fake.pyfloat(min_value=-100, max_value=100, right_digits=2) for _ in range(size)]


@dataclass
class TestSerie:
    name: str
    data: list[float]
    mean: float | None
    median: float | None
    q_1: float | None
    q_3: float | None

    @staticmethod
    def generate_constant(size : int) -> 'TestSerie':
        name = f"{size}_sample_number_Constant"
        data = [1.0 for _ in range(size)]
        metric = 1.0 if size > 0 else None
        return TestSerie(name, data, metric, metric, metric, metric)


    @staticmethod
    def generate_test_serie(fake : Faker, size: int) -> 'TestSerie':
        name = f"{size}_sample_number_{fake.unique.first_name()}"
        if size == 0:
            return TestSerie(name, [], None, None, None, None)

        data = generate_random_float_array(fake, size)

        mean = np.mean(data)
        median = np.median(data)
        q_1 = np.quantile(data, 0.25, method="inverted_cdf")
        q_3 = np.quantile(data, 0.75, method="inverted_cdf")
        return TestSerie(name, data, float(mean), float(median), float(q_1), float(q_3))
    
    def to_json(self) -> str:
        return json.dumps(asdict(self), indent=4)

    def save_to_json(self, folder: str) -> None:
        file_path = os.path.join(folder, f"{self.name}.json")
        with open(file_path, "w") as f:
            f.write(self.to_json())
            

OUTPUT_FOLDER = "plot_helper/tests/ressources/stats_series/data/"
NUMBER_OF_SERIES = 5

def get_argv_parser():
    parser = argparse.ArgumentParser(description='Generate a stats series')
    # Add the mutually exclusive group
    group = parser.add_mutually_exclusive_group()
    group.add_argument('-s', '--size', type=int, default=10, help="Size of the serie")
    group.add_argument('-m', '--multiple', action='store_true', help="Generate a lot of series, and store it in a folder. Disable the size argument")
    return parser

if __name__ == "__main__":
    parser = get_argv_parser()
    args = parser.parse_args()
    if args.multiple:
        fake = Faker(['en_US'])
        Faker.seed(42)
        os.makedirs(OUTPUT_FOLDER, exist_ok=True)
        for i in range(0, 11):
            for _ in range(NUMBER_OF_SERIES):
                test_serie = TestSerie.generate_test_serie(fake, i)
                test_serie.save_to_json(OUTPUT_FOLDER)
            TestSerie.generate_constant(i).save_to_json(OUTPUT_FOLDER)
    else:
        fake = Faker(['en_US'])
        test_serie = TestSerie.generate_test_serie(fake, size=args.size)
        print(test_serie.to_json())
    