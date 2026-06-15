struct Aircraft{
  name:String,weight:f32,wingarea:f32,lmaxco:f32,weiforce:f32,liftforce:f32,v:f32,liftneeded:f32
}

struct Airport{
  elevation:f32,p:f32
}

fn aeroden(h:f32)->f32{
  const P0:f32=1.225;
  let temph:f32=-h/8500.0;
  return P0*temph.exp();
}

fn compute_lift(p:f32,v:f32,s:f32,c:f32)->f32{
  return 0.5*p*(v*v)*s*c;
}
fn compute_takeoff_v(w:f32,p:f32,s:f32,c:f32)->f32{
  return ((2.0*w)/(p*s*c)).sqrt();
}
fn is_airborne(name:String,lift:f32,w:f32){
  if lift>=w {println!("The {} is Airborne",name)}
  else {println!("The {} is at Ground",name)}
}

fn main(){
  let mut cessna=Aircraft{name:String::from("Cessna 172"),weight:1111.0,wingarea:16.2,lmaxco:1.5,weiforce:0.0,liftforce:0.0,v:0.0,liftneeded:0.0};
  let mut cbe=Airport{elevation:411.0,p:0.0};
  cbe.p=aeroden(cbe.elevation);
  cessna.liftneeded=cessna.weight*9.81;
  cessna.v=compute_takeoff_v(cessna.liftneeded,cbe.p,cessna.wingarea,cessna.lmaxco);
  println!("takeoff v {}",cessna.v);
  cessna.liftforce=compute_lift(cbe.p,cessna.v,cessna.wingarea,cessna.lmaxco);
  println!("Air Density: {}",cbe.p);
  println!("Lift Force: {}",cessna.liftforce);
  is_airborne(cessna.name,cessna.liftforce,cessna.weiforce);
  
  
  
}