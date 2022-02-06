use std::collections::HashSet;
use std::io;
use itertools::Itertools;
use multimap::MultiMap;
use crate::distance::jaccard;
use crate::file_loading::Instance;
use crate::repartition::RepartitionClusters;

mod file_loading;
mod distance;
mod clustering;
mod repartition;


const FILE_NAME: &'static str = "NoSql.txt";

fn main() {
    let instances = file_loading::load_file(FILE_NAME)
        .expect(format!("Erreur lors de la lecture du fichier {FILE_NAME}").as_str());

    let repartition = clustering::dbscan(0.5, 4, &instances);
    
    println!("Nombre de clusters : {}",repartition.clusters().len());
    println!("Nombre de bruit : {}", repartition.bruit.len());

    let moyenne_jacquard: f32 = instances.iter()
        .cartesian_product(instances.iter())
        .filter(|(l, r)| l != r)
        .map(|(left, right)| jaccard(left, right))
        .sum::<f32>() / instances.len().pow(2) as f32;

    println!("Moyenne des distances : {moyenne_jacquard}");


    loop {
        match prompt(&repartition) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Erreure : {}", e);
            }
        }
    }
}

fn prompt(repartition: &RepartitionClusters) -> Result<(), &'static str> {
    let mut input = String::new();
    
    println!("Entrez le cluster a regarder parmis {}", repartition.clusters().len());
    //lecure de ligne
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Erreur de lecture")?;


    //transformation en chiffre entre -1 et nb cluster
    let num = input.trim()
        .parse::<isize>()
        .map_err(|_| "Veuillez entrer un chiffre")
        .and_then(|nm| {
            if (-1..repartition.clusters().len() as isize).contains(&nm)
            { Ok(nm) } 
            else
            { Err("Le numéro doit etre entre -1 et le nb de clusters") }
        })?;
    
    if num == -1{
        println!("Liste des points de bruit");
        for x in &repartition.bruit {
            println!("{x}");
        }
        Ok(())
    } else {
        let cluster = repartition.get_cluster_by_id(num.try_into().unwrap())
            .and_then(|head| repartition.get_cluster_vec(head))
            .ok_or("Erreur lors de la récupération du cluster numéroté")?;


        println!("{}", cluster.iter().map(|i|format!("{}",i)).join("\n"));

        
        let mots_diffs : HashSet<&String> = cluster.iter().flat_map(|i|&i.proprietes).collect();
        println!("Mots différents : [{}]",mots_diffs.iter().join(";"));
        
        
        println!("Cluster {num} affiché, contenant {} valeurs",cluster.len());

        Ok(())
    }
    

    
}

