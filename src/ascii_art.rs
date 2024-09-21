#[derive(Debug)]
pub struct Animation {
    index: u8,
    delay: u64,
}
impl Default for Animation {
    fn default() -> Self {
        Self {
            index: 0,
            delay: 1000,
        }
    }
}
impl Animation {
    pub fn next(&mut self) {
        self.index = (self.index + 1) % 7;
        
    }
    pub fn delay(&self) -> u64 {
        self.delay
    }
    pub fn set_delay(&mut self, delay: u64) {
        self.delay = delay;
    }
    pub fn print_ascii_art(&self) -> String {
        let vector = vec![
            r#"

             ______   
            /  __  \  
           |  |  |  | 
           |  |  |  | 
           |  `--'  | 
            \______/  
                      
           
            "#.to_string(),
            r#"

             ______    __          ___      
            /  __  \  |  |        /   \     
           |  |  |  | |  |       /  ^  \    
           |  |  |  | |  |      /  /_\  \   
           |  `--'  | |  `----./  _____  \  
            \______/  |_______/__/     \__\ 
                                            
           
            "#.to_string(),
            r#"

             ______    __          ___   ____    ____  ___      
            /  __  \  |  |        /   \  \   \  /   / /   \     
           |  |  |  | |  |       /  ^  \  \   \/   / /  ^  \    
           |  |  |  | |  |      /  /_\  \  \_    _/ /  /_\  \   
           |  `--'  | |  `----./  _____  \   |  |  /  _____  \  
            \______/  |_______/__/     \__\  |__| /__/     \__\ 
                
            "#.to_string(),
            r#"
                                          .d0;                                                                               
                                        .,cdo,..                                                                             
                                       .looooll,'dO0Oc                                                                       
                                      .coooooo':.0NXx.                                                                       
                                     .:oooooool;c;cl.                                                                        
                                    .coooooooooooool.                                                                        
                   .:.           ..;ooooooooooooo:;c'.      ...''.',,''...                                                   
                   .WNx;..     .':ooooooooooooooo.::,Od:.:clooooo;looooooolc:;,...                                           
                    'WMM0l;'..;looooooooooooooooo.:'xX0d'ooooooool:oooooooooooooo:.,..                                       
                     .KMMW0Okc,c;oooooooooooooooo:c:;l:coooooooooolooooooooooooool;ool;'.                                    
                     .,lKM0xc:o:ooooooooooooooooooool'ooooooooooooooooooooooooooocloooool;..                                 
                   .'loo:lc:oooooooooooooooooooooo:::'ooooooooooooooooooooooooooooooooooooo:'.                               
                  .;ooooooooooooooooooooooooooooooc;.cooooooooooooooooooooooooooocoooooooooool;..                            
                 .:oooooc:cooooooooooooooooooooooo,oOo:looooooooooooooooooooooooooooooooooooooo';..                          
        ..      .cooooc'd..'ooooooooooooooooooooo;dNNN0.looooooooo:looooooooooooooooooooooooooocoo:..                        
        XKc.. .'loooooc...,'lcoooooooooooooooooc:.cccc:coooooooooocooooooooooooooooooooooooooooooooo:..                      
        kMWko.coooooooooc:co;ooooooooooooooooc;,;ldoooooooooooooooooooooooooooooooooooocoooooolooooooo:..                    
        .dxl:;oooooooooooooooooooooooooooooooccoooooooooooooooooooloooooooooooooooooooo:cooooooooooooooo:'.                  
       .loloolooooooooooooooooooooooolooooooooooooooooooooooooooooooooooooooooooooooool::cooooooooooooooool;..               
      .cooooooooooooooooooooooooooolcloooooooooooooooooooolloooooooooooool;looooooooooc:::loooooooooooooooolcc;'..           
      .oooollolc;,oooooooooooololcccooooooooooooocooooooool:looooooooooo:,oooooooooool:::':ccllloloooooooooooooooc:,'..      
       loo:,;;;:cooooooololc, ';:::cllolooooooooc,ooooooooo:cooooooooool'cloooooooool:::'  ;;::::ccccllllooolllllloool;.     
        .  .:coooooc::;            ,:::ccclloolo.coooloooooc:'ooooooooo,loooooooooolc::',..      .;:;::::::::ccc:;:;;        
                                     .,;::::::c;,ooooooooooc:,:llllllcc.ooooooooooc:;'':::..                                 
                                      ,,,,,;:::';ooooooooool:;'::::::::.oooooooolc:;',::::,..                                
                                      .:::;;,,;';ooooooooooo::.         cooooooo::;.::::::::'.                               
                                       ,:::::::;'oooooooolol;;.         ,ooooooo::',:::::::::.                               
                                        .:::::::.loooooooooo::.         .oooooooc:.'::::::::::.                              
                                        .,::::::,,oooooooool::.         .oooooooc:. .;::::::::;.                             
                                       ..:::::::;.oooooooooc::.         .oooooooc:,.,::::::::::.                             
                                        .''.    .'ooooooolc::;         .cooooooo::,':;;;;;,;,                                
                                                 ccllcc::;;;.             .;;:;'.                                            
         
            "#.to_string(),
        ];
        if self.index as usize >= vector.len() {
            return vector[3].clone();
        }
        vector[self.index as usize].clone()
    }
}

/*
                                           ...
                                          .d0;                                                                               
                                        .,cdo,..                                                                             
                                       .looooll,'dO0Oc                                                                       
                                      .coooooo':.0NXx.                                                                       
                                     .:oooooool;c;cl.                                                                        
                                    .coooooooooooool.                                                                        
                   .:.           ..;ooooooooooooo:;c'.      ...''.',,''...                                                   
                   .WNx;..     .':ooooooooooooooo.::,Od:.:clooooo;looooooolc:;,...                                           
                    'WMM0l;'..;looooooooooooooooo.:'xX0d'ooooooool:oooooooooooooo:.,..                                       
                     .KMMW0Okc,c;oooooooooooooooo:c:;l:coooooooooolooooooooooooool;ool;'.                                    
                     .,lKM0xc:o:ooooooooooooooooooool'ooooooooooooooooooooooooooocloooool;..                                 
                   .'loo:lc:oooooooooooooooooooooo:::'ooooooooooooooooooooooooooooooooooooo:'.                               
                  .;ooooooooooooooooooooooooooooooc;.cooooooooooooooooooooooooooocoooooooooool;..                            
                 .:oooooc:cooooooooooooooooooooooo,oOo:looooooooooooooooooooooooooooooooooooooo';..                          
        ..      .cooooc'd..'ooooooooooooooooooooo;dNNN0.looooooooo:looooooooooooooooooooooooooocoo:..                        
        XKc.. .'loooooc...,'lcoooooooooooooooooc:.cccc:coooooooooocooooooooooooooooooooooooooooooooo:..                      
        kMWko.coooooooooc:co;ooooooooooooooooc;,;ldoooooooooooooooooooooooooooooooooooocoooooolooooooo:..                    
        .dxl:;oooooooooooooooooooooooooooooooccoooooooooooooooooooloooooooooooooooooooo:cooooooooooooooo:'.                  
       .loloolooooooooooooooooooooooolooooooooooooooooooooooooooooooooooooooooooooooool::cooooooooooooooool;..               
      .cooooooooooooooooooooooooooolcloooooooooooooooooooolloooooooooooool;looooooooooc:::loooooooooooooooolcc;'..           
      .oooollolc;,oooooooooooololcccooooooooooooocooooooool:looooooooooo:,oooooooooool:::':ccllloloooooooooooooooc:,'..      
       loo:,;;;:cooooooololc, ';:::cllolooooooooc,ooooooooo:cooooooooool'cloooooooool:::'  ;;::::ccccllllooolllllloool;.     
        .  .:coooooc::;            ,:::ccclloolo.coooloooooc:'ooooooooo,loooooooooolc::',..      .;:;::::::::ccc:;:;;        
                                     .,;::::::c;,ooooooooooc:,:llllllcc.ooooooooooc:;'':::..                                 
                                      ,,,,,;:::';ooooooooool:;'::::::::.oooooooolc:;',::::,..                                
                                      .:::;;,,;';ooooooooooo::.         cooooooo::;.::::::::'.                               
                                       ,:::::::;'oooooooolol;;.         ,ooooooo::',:::::::::.                               
                                        .:::::::.loooooooooo::.         .oooooooc:.'::::::::::.                              
                                        .,::::::,,oooooooool::.         .oooooooc:. .;::::::::;.                             
                                       ..:::::::;.oooooooooc::.         .oooooooc:,.,::::::::::.                             
                                        .''.    .'ooooooolc::;         .cooooooo::,':;;;;;,;,                                
                                                 ccllcc::;;;.             .;;:;'.                                            
*/