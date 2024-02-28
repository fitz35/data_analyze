"""
This script reads the data from the folder data and generates the macro in rust to get the all_stats struct
"""

from dataclasses import dataclass
import os


INPUT_FOLDER = "plot_helper/tests/ressources/stats_series/data/"
TEST_RELATIVE_PATH = "tests/ressources/stats_series/data/"

@dataclass
class StatsInfo:
    file_path : str
    name : str
    lowercase_name : str
    size : int

    @staticmethod
    def get_stats_info_from_file(file_path: str) -> 'StatsInfo':
        """
        get the information from the file name. The file name is to the format : "size_sample_number_name.json"

        return (size, name, name in lowercase)
        """
        file_name = os.path.basename(file_path)
        file_name = file_name[:-5]
        file_splitted = file_name.split("_")
        size = file_splitted[0]
        name = file_splitted[-1]
        return StatsInfo(file_path, name, name.lower(), int(size))

def main():
    data : dict[int, list[StatsInfo]] = {}
    files = os.listdir(INPUT_FOLDER)
    files.sort()
    for file in files:
        if file.endswith(".json"):
            file_path = os.path.join(TEST_RELATIVE_PATH, file)
            stats_info = StatsInfo.get_stats_info_from_file(file_path)
            if stats_info.size not in data:
                data[stats_info.size] = []
            data[stats_info.size].append(stats_info)

    keys = list(data.keys())
    keys.sort()
    i_key = 0
    print("generate_plot_key!(", end="\n")
    for size in keys:
        stats_infos = data[size]
        print(f"\tKey{size}Sample {{", end="\n")
        i = 0
        for stats_info in stats_infos:
            # add the size if we have a constant
            attribut_name = stats_info.lowercase_name
            if stats_info.name == "Constant":
                attribut_name = f"{attribut_name}{size}"
            print(f"\t\t{stats_info.name}, {attribut_name}, \"{stats_info.file_path}\"", end="")
            if i < len(stats_infos) - 1:
                print(",", end="\n")
            else:
                print(end="\n")
            i+=1

        print("\t}", end="")
        if i_key < len(keys) - 1:
            print(",", end="\n")
        else:
            print(end="\n")
        i_key += 1
        
    print(");")


if __name__ == '__main__':
    main()