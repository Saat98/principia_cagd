use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point{
    x: u32,
    y: u32,
}

// Define a data structure for a triangle.
#[derive(Debug, Clone, Copy)]
struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point
}

fn main() {
    let n: u32 = 2; // Size of the grid (n x n)

    // Calculate length of a single square in the grid
    let square_length = 1.0 / n as f64;
    let mut triangles = Vec::new();

    // Create smaller squares
    for i in 0..n {
        for j in 0..n {
            if (i+j)%2==0{
                let triangle1 = Triangle {
                    p1: Point{x:i,y:j},
                    p2: Point{x:i,y:j+1 },
                    p3: Point{x:i+1 ,y:j },
                };
    
                let triangle2 = Triangle {
                    p1: Point{x:i ,y:j+1 },
                    p2: Point{x:i+1 ,y:j },
                    p3: Point{x:i+1 ,y:j+1 },
                };
                triangles.push(triangle1);
                triangles.push(triangle2);
            }else{
                let triangle1 = Triangle {
                    p1: Point{x:i ,y:j },
                    p2: Point{x:i+1 ,y:j },
                    p3: Point{x:i+1 ,y:j+1 },
                };
    
                let triangle2 = Triangle {
                    p1: Point{x:i ,y:j },
                    p2: Point{x:i ,y:j+1 },
                    p3: Point{x:i+1 ,y:j+1 },
                };
                triangles.push(triangle1);
                triangles.push(triangle2);
            };
        }
    }

    let mut mat = DMatrix::<f64>::zeros((2*3*n*n).try_into().unwrap(), (2*3*n*n).try_into().unwrap());

    // Print the generated triangles
    // let mut cont=0;
    for i in 0..2*n*n {
        let i_ind: usize=i.try_into().unwrap();

        // Definimos el polinomio 3*i como el "grande" (aquel que crece hacia el ángulo recto).
        mat[(3*i_ind,3*i_ind)]=1.0/3.0*square_length*square_length;
        mat[(3*i_ind,3*i_ind+1)]=5.0/48.0*square_length*square_length;
        mat[(3*i_ind,3*i_ind+2)]=5.0/48.0*square_length*square_length;
        mat[(3*i_ind+1,3*i_ind)]=5.0/48.0*square_length*square_length;
        mat[(3*i_ind+1,3*i_ind+1)]=1.0/12.0*square_length*square_length;
        mat[(3*i_ind+1,3*i_ind+2)]=1.0/24.0*square_length*square_length;
        mat[(3*i_ind+2,3*i_ind)]=5.0/48.0*square_length*square_length;
        mat[(3*i_ind+2,3*i_ind+1)]=1.0/24.0*square_length*square_length;
        mat[(3*i_ind+2,3*i_ind+2)]=1.0/12.0*square_length*square_length;

        /*if triangles[i_ind].p1.x==0{
            if triangles[i_ind].p2.x==0{
                // Definimos el triángulo 3*i+1 como el polinomio de "arriba"
                mat[(3*i_ind,3*i_ind)]
            }
        }*/


        // for j in 0..2*n*n{
        //     if i==j{continue;}

        //     let j_ind: usize=j.try_into().unwrap();
        //     if triangles[i_ind].p1==triangles[j_ind].p1{
        //         println!("{} {}",3*i_ind,3*j_ind);
        //         cont=cont+1;
        //         mat[(3*i_ind,3*j_ind)]=1.0;
        //     }else if triangles[i_ind].p1==triangles[j_ind].p2{
        //         println!("{} {}",3*i_ind,3*j_ind+1);
        //         cont=cont+1;
        //         mat[(3*i_ind,3*j_ind+1)]=1.0;
        //     }else if triangles[i_ind].p1==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind,3*j_ind+2)]=1.0;
        //     }else if triangles[i_ind].p2==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind+1,3*j_ind+2)]=1.0;
        //     }else if triangles[i_ind].p2==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind+1,3*j_ind+2)]=1.0;
        //     }else if triangles[i_ind].p2==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind+1,3*j_ind+2)]=1.0;
        //     }
        //     else if triangles[i_ind].p3==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind+2,3*j_ind+2)]=1.0;
        //     }
        //     else if triangles[i_ind].p3==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind+2,3*j_ind+2)]=1.0;
        //     }
        //     else if triangles[i_ind].p3==triangles[j_ind].p3{
        //         println!("{} {}",i_ind,3*j_ind+2);
        //         cont=cont+1;
        //         mat[(3*i_ind+2,3*j_ind+2)]=1.0;
        //     }
        // }
    }
    //println!("{} {}",mat.determinant() as f64,cont);
    print!("{}",mat);
}