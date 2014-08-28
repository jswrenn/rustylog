pub enum Term {
	Atom(String),
	Variable(String),
	Structure(String, Vec<Term>),
}
 
pub type Predicate
	= Term;
	   
pub type Rule
	= (Predicate, Vec<Predicate>);
	   
pub type Query
	= Predicate;
   
pub type Program
	= Vec<Rule>;
		


