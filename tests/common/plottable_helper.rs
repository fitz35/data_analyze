/**
 * This file define a struct with data from json files, and made a serie of unit tests to check the data integrity
 * and some of basic operations
 * 
 */


use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::fmt::{Display, Formatter};
use plot_helper::data::plottable::{SerieKey, Plottable};
use plot_helper::stats::stats_serie::MetricName;
use plot_helper::data::filtering::Mask;


use crate::common::stats_helper::TestSerie;

/// get a test serie from a json file
pub trait TestSerieGetter<Key> where Key : SerieKey {
    fn get_test_serie(&self, key : &Key) -> TestSerie;
}

macro_rules! generate_plot_key {
    ($($key_name:ident { $($variant:ident, $struct_name:ident, $json_path:literal),* }),*) => {
        $(
            #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy)]
            pub enum $key_name {
                $($variant),*
            }

            /// implement the SerieKey trait for the key
            impl SerieKey for $key_name {
                fn get_display_name(&self) -> String {
                    match self {
                        $(Self::$variant => {
                            let path_o = Path::new($json_path);
                            let file_name = path_o.file_name().unwrap().to_str().unwrap();
                            file_name.to_string()
                        })*
                    }
                }

                fn is_numeric(&self) -> bool {
                    match self {
                        $(Self::$variant => true),*
                    }
                }

                fn is_string(&self) -> bool {
                    false
                }

                fn is_object(&self) -> bool {
                    false
                }
            }

            impl Display for $key_name {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.get_display_name())
                }
            }

            impl std::cmp::PartialOrd for $key_name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.get_display_name().partial_cmp(&other.get_display_name())
                }
            }
            
            impl std::cmp::Ord for $key_name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.get_display_name().cmp(&other.get_display_name())
                }
            }
        )*

        /// Struct that contains all the stats from the json files
        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub struct AllStats {
            $(
                $($struct_name : Vec<f32>),*
            ),*
        }

        impl AllStats {
            pub fn new() -> Self {
                $(
                    $(
                        let $struct_name = {
                            let test_serie = TestSerie::new($json_path);
                            test_serie.data
                        };
                    )*
                    

                )*

                Self {
                    $(
                        $($struct_name),*
                    ),*
                }
            }
        }

        $(
            // all the keys are plottable, and numeric
            impl Plottable<$key_name> for AllStats {
                fn get_numeric_series(&self, key : &$key_name) -> Vec<f32> {
                    match key {
                        $($key_name::$variant => {
                            self.$struct_name.clone()
                        }),*
                    }
                }

                fn get_string_series(&self, _key : &$key_name) -> Vec<String> {
                    panic!("Not a string serie");
                }

                fn get_number_of_samples(&self) -> usize {
                    let mut res = 0;
                    $(
                        res = std::cmp::max(res, self.$struct_name.len());
                    )*
                    res
                }
            }
        )*

        $(
            /// helper to get the test serie from the json files
            impl TestSerieGetter<$key_name> for AllStats {
                fn get_test_serie(&self, key : &$key_name) -> TestSerie {
                    match key {
                        $($key_name::$variant => {
                            serde_json::from_str(fs::read_to_string($json_path).unwrap().as_str()).unwrap()
                        }),*
                    }
                }
            }
        )*

        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                $(

                    paste::item! {
                        /// test the data integrity of the serie
                        #[test]
                        fn [< dummy_mask_$struct_name _test>]() -> Result<(), Box<dyn std::error::Error>>{
                            let all_stats = AllStats::new();
                            let data_size = Plottable::<$key_name>::get_number_of_samples(&all_stats);
                            let dummy_data = vec![0.0; data_size];
                            let filter_mask : Mask = Plottable::<$key_name>::combine_filter(&all_stats, &None, &None);
                            let filtered_data = filter_mask.apply(&dummy_data);

                            assert_eq!(filtered_data.len(), data_size);
                            for d in filtered_data {
                                assert_eq!(d, 0.0);
                            }
                            Ok(())
                        }

                    }
                    paste::item! {
                        /// test the data integrity of the serie
                        #[test]
                        fn [< data_integrity_$struct_name _test>]() -> Result<(), Box<dyn std::error::Error>>{
                            let all_stats = AllStats::new();
                            let data_size = Plottable::<$key_name>::get_number_of_samples(&all_stats);
                            let test_serie = all_stats.get_test_serie(&$key_name::$variant);

                            assert_eq!(test_serie.data.len(), data_size);


                            Ok(())
                        }
                    }
                    paste::item! {
                        /// test the aggregation of the serie
                        #[test]
                        fn [< aggregate_$struct_name _test>]() -> Result<(), Box<dyn std::error::Error>>{
                            let all_stats = AllStats::new();
                            let legends = vec!["all".to_string(); Plottable::<$key_name>::get_number_of_samples(&all_stats)];
                            let aggregation = 
                                all_stats.aggregate(
                                    &$key_name::Constant, 
                                    &$key_name::$variant, 
                                    &None, 
                                    &legends, 
                                    &None, 
                                    Some(MetricName::Mean),
                                )?;
                            let test_serie = all_stats.get_test_serie(&$key_name::$variant);
                            let data = aggregation.get_data();

                            if test_serie.data.len() == 0 {
                                assert_eq!(data.len(), 0);
                            } else {
                                assert_eq!(data.len(), 1);
                                assert!(data.contains_key("all"));
                                assert_eq!(data["all"].len(), 1);

                                let point = data["all"][0];
                                assert_eq!(point.0, 1.0);
                                assert_abs_diff_eq!(point.1 as f64, test_serie.mean.unwrap(), epsilon = 0.0001);
                            }

                            Ok(())
                        }
                    }
                
                )*
            )*

        }
    };
}

generate_plot_key!(
    Key0Sample {
            Angel, angel, "tests/ressources/stats_series/data/0_sample_number_Angel.json",
            Constant, constant0, "tests/ressources/stats_series/data/0_sample_number_Constant.json",
            Danielle, danielle, "tests/ressources/stats_series/data/0_sample_number_Danielle.json",
            Jeffrey, jeffrey, "tests/ressources/stats_series/data/0_sample_number_Jeffrey.json",
            Jill, jill, "tests/ressources/stats_series/data/0_sample_number_Jill.json",
            Joshua, joshua, "tests/ressources/stats_series/data/0_sample_number_Joshua.json"
    },
    Key1Sample {
            Brittany, brittany, "tests/ressources/stats_series/data/1_sample_number_Brittany.json",
            Carolyn, carolyn, "tests/ressources/stats_series/data/1_sample_number_Carolyn.json",
            Clayton, clayton, "tests/ressources/stats_series/data/1_sample_number_Clayton.json",
            Constant, constant1, "tests/ressources/stats_series/data/1_sample_number_Constant.json",
            Erica, erica, "tests/ressources/stats_series/data/1_sample_number_Erica.json",
            Jennifer, jennifer, "tests/ressources/stats_series/data/1_sample_number_Jennifer.json"
    },
    Key2Sample {
            Constant, constant2, "tests/ressources/stats_series/data/2_sample_number_Constant.json",
            Jason, jason, "tests/ressources/stats_series/data/2_sample_number_Jason.json",
            Katherine, katherine, "tests/ressources/stats_series/data/2_sample_number_Katherine.json",
            Lindsay, lindsay, "tests/ressources/stats_series/data/2_sample_number_Lindsay.json",
            Noah, noah, "tests/ressources/stats_series/data/2_sample_number_Noah.json",
            Robert, robert, "tests/ressources/stats_series/data/2_sample_number_Robert.json"
    },
    Key3Sample {
            Constant, constant3, "tests/ressources/stats_series/data/3_sample_number_Constant.json",
            Cynthia, cynthia, "tests/ressources/stats_series/data/3_sample_number_Cynthia.json",
            Donna, donna, "tests/ressources/stats_series/data/3_sample_number_Donna.json",
            Joseph, joseph, "tests/ressources/stats_series/data/3_sample_number_Joseph.json",
            Nicole, nicole, "tests/ressources/stats_series/data/3_sample_number_Nicole.json",
            Teresa, teresa, "tests/ressources/stats_series/data/3_sample_number_Teresa.json"
    },
    Key4Sample {
            Amanda, amanda, "tests/ressources/stats_series/data/4_sample_number_Amanda.json",
            Brenda, brenda, "tests/ressources/stats_series/data/4_sample_number_Brenda.json",
            Carl, carl, "tests/ressources/stats_series/data/4_sample_number_Carl.json",
            Constant, constant4, "tests/ressources/stats_series/data/4_sample_number_Constant.json",
            Mark, mark, "tests/ressources/stats_series/data/4_sample_number_Mark.json",
            Stephen, stephen, "tests/ressources/stats_series/data/4_sample_number_Stephen.json"
    },
    Key5Sample {
            Caitlin, caitlin, "tests/ressources/stats_series/data/5_sample_number_Caitlin.json",
            Chelsea, chelsea, "tests/ressources/stats_series/data/5_sample_number_Chelsea.json",
            Constant, constant5, "tests/ressources/stats_series/data/5_sample_number_Constant.json",
            Karen, karen, "tests/ressources/stats_series/data/5_sample_number_Karen.json",
            Katelyn, katelyn, "tests/ressources/stats_series/data/5_sample_number_Katelyn.json",
            Nancy, nancy, "tests/ressources/stats_series/data/5_sample_number_Nancy.json"
    },
    Key6Sample {
            Adrian, adrian, "tests/ressources/stats_series/data/6_sample_number_Adrian.json",
            Constant, constant6, "tests/ressources/stats_series/data/6_sample_number_Constant.json",
            Rachel, rachel, "tests/ressources/stats_series/data/6_sample_number_Rachel.json",
            Tamara, tamara, "tests/ressources/stats_series/data/6_sample_number_Tamara.json",
            Tiffany, tiffany, "tests/ressources/stats_series/data/6_sample_number_Tiffany.json",
            Tracy, tracy, "tests/ressources/stats_series/data/6_sample_number_Tracy.json"
    },
    Key7Sample {
            Constant, constant7, "tests/ressources/stats_series/data/7_sample_number_Constant.json",
            James, james, "tests/ressources/stats_series/data/7_sample_number_James.json",
            Michael, michael, "tests/ressources/stats_series/data/7_sample_number_Michael.json",
            Todd, todd, "tests/ressources/stats_series/data/7_sample_number_Todd.json",
            Tricia, tricia, "tests/ressources/stats_series/data/7_sample_number_Tricia.json",
            Yvette, yvette, "tests/ressources/stats_series/data/7_sample_number_Yvette.json"
    },
    Key8Sample {
            Brandy, brandy, "tests/ressources/stats_series/data/8_sample_number_Brandy.json",
            Constant, constant8, "tests/ressources/stats_series/data/8_sample_number_Constant.json",
            Justin, justin, "tests/ressources/stats_series/data/8_sample_number_Justin.json",
            Melissa, melissa, "tests/ressources/stats_series/data/8_sample_number_Melissa.json",
            Sarah, sarah, "tests/ressources/stats_series/data/8_sample_number_Sarah.json",
            Tanya, tanya, "tests/ressources/stats_series/data/8_sample_number_Tanya.json"
    },
    Key9Sample {
            Anita, anita, "tests/ressources/stats_series/data/9_sample_number_Anita.json",
            Constant, constant9, "tests/ressources/stats_series/data/9_sample_number_Constant.json",
            Kristin, kristin, "tests/ressources/stats_series/data/9_sample_number_Kristin.json",
            Maria, maria, "tests/ressources/stats_series/data/9_sample_number_Maria.json",
            Pamela, pamela, "tests/ressources/stats_series/data/9_sample_number_Pamela.json",
            Rebecca, rebecca, "tests/ressources/stats_series/data/9_sample_number_Rebecca.json"
    },
    Key10Sample {
            Chad, chad, "tests/ressources/stats_series/data/10_sample_number_Chad.json",
            Constant, constant10, "tests/ressources/stats_series/data/10_sample_number_Constant.json",
            Elizabeth, elizabeth, "tests/ressources/stats_series/data/10_sample_number_Elizabeth.json",
            Samuel, samuel, "tests/ressources/stats_series/data/10_sample_number_Samuel.json",
            Scott, scott, "tests/ressources/stats_series/data/10_sample_number_Scott.json",
            Tammy, tammy, "tests/ressources/stats_series/data/10_sample_number_Tammy.json"
    }
);