use std::collections::{HashMap, HashSet};
use multimap::MultiMap;
use crate::file_loading::Instance;
use crate::jaccard;
use crate::repartition::RepartitionClusters;

pub fn dbscan(epsilon: f32, min_pts: usize, points: &Vec<Instance>) -> RepartitionClusters
{
    let mut repartition = RepartitionClusters::new();

    let params = DbScanParams {
        epsilon,
        min_pts,
        points,
    };

    for point in points {
        dbscan_inner(&params, point,None, &mut repartition);
    }
    

    repartition
}

struct DbScanParams<'a> {
    epsilon: f32,
    min_pts: usize,
    points: &'a Vec<Instance>,
}



fn dbscan_inner<'r>(params: &DbScanParams<'r>, 
                    actuel: &'r Instance, 
                    actuel_cluster: Option<&'r Instance>,
                    repartition: &mut RepartitionClusters<'r>)
{
    if repartition.deja_visite(&actuel) { return; };
    
    let voisins: Vec<&Instance> = params.points.iter()
        .filter(|p| !repartition.deja_visite(p))
        .filter(|p| {
            jaccard(actuel, p) <= params.epsilon
        })
        .collect();
    
    match (actuel_cluster, voisins.len() >= params.min_pts) {
        (None, true) => {
            repartition.create_cluster(actuel);
            for x in voisins {
                dbscan_inner(params, x, Some(actuel), repartition);
            }
        }
        (Some(actuel_cluster), true) => {
            //Core dans un cluster, on continue
            repartition.add_to_cluster(actuel_cluster, actuel);
            for x in voisins {
                dbscan_inner(params, x, Some(actuel_cluster),repartition);
            }
        }
        (None, false) => {
            //points nul;
            repartition.bruit.insert(actuel);
        }
        (Some(actuel_cluster), false) => {
            //Bordure de cluster on arrete
            repartition.add_to_cluster(actuel_cluster, actuel);
        }
    }
}
