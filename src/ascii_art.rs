#[derive(Debug)]
pub struct Animation {
    index: u8,
    delay: u64,
    ascending: bool,
}
impl Default for Animation {
    fn default() -> Self {
        Self {
            index: 0,
            delay: 1000,
            ascending: true,
        }
    }
}
impl Animation {
    pub fn next_olaya(&mut self) {
        self.index = (self.index + 1) % 7;
        
    }
    pub fn print_os_name(&self) -> String {
        let vector = vec![
            r#"

     ██╗ █████╗ ██╗     ███████╗████████╗███████╗██████╗      ██████╗ ███████╗
     ██║██╔══██╗██║     ██╔════╝╚══██╔══╝██╔════╝██╔══██╗    ██╔═══██╗██╔════╝
     ██║███████║██║     ███████╗   ██║   █████╗  ██████╔╝    ██║   ██║███████╗
██   ██║██╔══██║██║     ╚════██║   ██║   ██╔══╝  ██╔══██╗    ██║   ██║╚════██║
╚█████╔╝██║  ██║███████╗███████║   ██║   ███████╗██║  ██║    ╚██████╔╝███████║
 ╚════╝ ╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝     ╚═════╝ ╚══════╝
                                                                              
            "#.to_string(),
        ];
        vector[0].clone()
    }
    pub fn next(&mut self) {
        if self.ascending {
            self.index += 1;
            if self.index == 4 {
                self.ascending = false; // switch to descending
            }
        } else {
            self.index -= 1;
            if self.index == 0 {
                self.ascending = true; // switch to ascending
            }
        }
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
                  _________-----_____
       _____------           __      ----_
___----             ___------              \
   ----________        ----            ___  \
               -----__    |           / _____)
                    __-              / /     \
          _____-----    ___--          \  0 /)\
      ----_____      ---____            \__/  /
               -----__    \ --    _          /\
                      --__--__     \_____/   \_/\
                              ----|   /          |
                                  |  |___________|
                                  |  | ((_(_)| )_)
                                  |  \_((_(_)|/)_)                    
                                  \             (
                                   \_____________)
            "#.to_string(),
            r#"
                  _________-----_____
       _____------           __      ----_
 __----             ___------              \
   ----________        ----            ___  \
               -----__    |           / _____)
                    __-                /     \
          _____-----    ___--          \  | /)\
      ----_____      ---____            \__/  /
               -----__    \ --    _          /\
                      --__--__     \_____/   \_/\
                              ----|   /          |
                                  |  |_((_(_)| )_)
                                  |  | 
                                  |  \_((_(_)|/)_)                    
                                  \             (
                                   \_____________)
            "#.to_string(),
            r#"
                  _________-----_____
          __------           __      ----_
  ____----          ___------              \
      ----_____        ----           _____ \
               -----__    |             _____)
                    __-                /     \
        _______-----    ___--          \  - /)\                  
  ------_______      ---____            \__/  /                 
               -----__    \ --    _          /\                  
                      --__--__     \_____/   \_/\                
                              ----|   /          |                \ /
                                  |  |_((_(_)| )_)____________  ( /  )
                                  |  |          ()_)__________)))))))
                                  |  \_((_(_)|/(_)
                                  \             (
                                   \_____________)


            "#.to_string(),
            r#"
                  _________-----_____
          __------           __      ----_
   ___----          ___------              \
      ----_____        ----           _____ \
               -----__    |             _____)
                    __-                /     \
        _______-----    ___--          \  - /)\                  
   -----_______      ---____            \__/  /                 
               -----__    \ --    _          /\                  \ \_
                      --__--__     \_____/   \_/\                (   )
                              ----|   /          |                \ /
                                  |  |_((_(_)| )_)____________  ( /  )
                                  |  |          ()_)__________)))))))
                                  |  \_((_(_)|/(_)
                                  \             (
                                   \_____________)


            "#.to_string(),
            r#"
                  _________-----_____
          __------           __      ----_
   ___----          ___------              \
      ----_____        ----          \_____ \
               -----__    |             _____)
                    __-                /     \
        _______-----    ___--          \  * /)\                  |  /
     ---_______      ---____            \__/  /                 (/ |)
               -----__    \ --    _          /\                  \ \_
                      --__--__     \_____/   \_/\                (   )
                              ----|   /          |                \ /
                                  |  |_((_(_)| )_)____________  ( /  )
                                  |  |          ()_)__________)))))))
                                  |  \_((_(_)|/(_)
                                  \             (
                                   \_____________)


            "#.to_string(),
        ];
        if self.index as usize >= vector.len() {
            return vector[4].clone();
        }        vector[self.index as usize].clone()
    }
    pub fn print_ascii_art_bomb(&self) -> String {
        let vector = vec! [
            r#"
                    ..   .                  
                .:oc;,::;';::'            
                'coc:::::::xOc            
                .::;;;;::::ol'            
                .,::,,,,;:coddl,           
                .';,,,,,;:okOko,           
                ...........  
            "#.to_string(),
            r#"
                    ...                         
                ,:coodol;..                   
            .:c' '::oO00000Oo,                  
            ''. ':::coxO000Oc'                 
            ..,col:::::ccc:;.                 
            ,::d00d:::cc:;;,.                  
            ':::llc::oO00Okxl,                  
            .,:lxc;;:d000000Oc'                 
            .,:;,;::okO00Oo:.                 
                .',;;:::c:;,'                  
                    ',,,,,'..                   
                    .,.                         
                    .,                          
                    .,                          
                    ,,.                         
                    .,,'                         
                    ',,,.                        
                    ''''.                        
                    .... 

            "#.to_string(),
            r#"
                   ....                           
                 ,:ldddol;..                      
            ..  .::d000000kd:.                    
            ''. ,cc:dO000000Oo;.                  
            .;,,:col:ldkOO0Okl:,                  
           ,dOx::d0Ol:::codoc::,                  
          'lOOd::doc::lxO000x::.                  
          .:lc;;;::::ck0Okddddo:.                 
           ';,'...,;:::cc::oO00x:.                
            ...   ';::::::::lool:;                
                 ';:::::::::;,'..                 
                 ';:::::;..                       
                  .;;;;.                          
                   .,,.                           
                    ',.                           
                    ''.                           
                   .,'.                           
                   ''''.                          
                   .... 

            "#.to_string(),
            r#"3
                    .',;;;,.                
                    ;:::::::::.        ..    
            .'.  ;:::lkkkkkxdc.    ,ol.   
                .   ;:::cxO000000l:'  ':,    
            ;lc'';;;;::clodxdl:::. .      
            :O0d:,....,;:dO000Ol::.        
            'l0d:.      ';ldoooolll:,.      
            ':lc.        .;:cl:cxO0d;,'     
            .          .;:ll:::cc:;,,.    
                    ...'cxkl:''::;,,,.    
                    ',;::::;'. .,,'..      
                    .;:'                  
                        ':;.                
                        .;:;'                
                    .:;,.                 
                    .;,.                  
                    .:;.                  
                    .;,,.                 
                    .,,,.                 
                    ....  

            "#.to_string(),
            r#"
                     ...'''.      .;'             
                   .;::cl:::,    ;:dl             
               '.  .:::lxOkxo;.. .,,.             
                    '::k000kxkxl:;'               
              .;c:;,,:::cc:::::::::,              
             .:oo::,.....,,cdxxdc::;              
             .::::.      .;cdxolc::,.             
            .:;...        ,;::::::oxxo;.          
           .;:;....        .;::,,::ldxo:.         
            .,,,,'.  .....',;:.   '::::,          
                    ,ll:::::::.    ;;,.           
                    ;::;....'.                    
                     .,::,..                      
                        '::,.                     
                       .';,.                      
                    .',,,.                        
                    .,,,.                         
                     ',,'                         
                     .'''.                      
            "#.to_string(),
            r#"
               ..',:lol:'                         
        ...   .::::lddxl:,                        
        .,.       .:ck00d:,.                      
       .         .,::cll:::::;,. .'               
      .:..'.    .''.....',locloc'                 
       ..:dc.            .:oc;.                   
         ;:.             .,;::,                   
         .. ...          ,;,'',;'....             
         ,::::;          '.     ;::co:.           
           ...  ';'.  .''.     .;lxOx:.           
               .:do::,;::'      .,:;'             
                .;;;'                             
                  ..,'...                         
                    .,,,,                         
                     ...                          
                 ';.                              
                 ;:;,.                            
                  ..':.                           
                     .  
            "#.to_string(),
            r#"
                ....                              
        ..  .',;;:ol,                             
            .   ';:::'                            
               ...',,:clc;'.                      
                     .:lodl:.                     
     .,,..            .:. .,.                     
     .,,.             ':;.                        
    .'....           .;..'.                       
    .'     ..        ..     .,;,                  
     ..  ..,'               .;::'                 
        .... ..   .,;.   .  .;ldl'                
             ;:'   ...  .;','.'''.                
              .:'                                 
               ..'''.                             
                  .::.                            
                 ...                              
            .;,                                   
             .,,;'                                
                ;,   
            "#.to_string(),
            ];
    vector[self.index as usize].clone()
    }
    pub fn print_ascii_art_olaya(&self) -> String {
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