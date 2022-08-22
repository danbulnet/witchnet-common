pub trait Sensor {
    type ElementType;
    type DataType;

    fn name(&self) -> &str;
    
    fn new(name: &str) -> Self;
    
    fn insert<'graph, 'point>(
        &'graph mut self, item: &'point Self::DataType
    ) -> &'graph Self::ElementType;
    
    fn search<'graph, 'point>(
        &'graph self, item: &'point Self::DataType
    ) -> Option<&'graph Self::ElementType>;
}