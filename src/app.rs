// use fmtsize::{Conventional, FmtSize};
use leptos::{html::Input, *};
use leptos::ev::SubmitEvent;
use leptos_meta::*;
use leptos_router::*;
#[cfg(feature = "ssr")]
use ammonia::clean;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html />
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pkg/shareboxx.css"/>

        // sets the document title
        <Title text="LIMINAL JOURNEY"/>
        <Stylesheet id="leptos" href="/pkg/shareboxx.css"/>
        <Script src="/assets/abstractglitch.js"></Script>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Splash/>
                    <Route path="/home" view=HomePage/>
                    <Route path="/*any" view=Splash/>
                    
                </Routes>
            </main>
        </Router>
    }
}


/// Renders the home page of your application.
#[component]
fn Splash() -> impl IntoView {
    // let (path, set_path) = create_signal("".to_string());
    view! {

        <div id="maincontainer" class="bg-red" style="min-height: 100vh;" >

            <a href="/home">splash</a>
            </ br>
            <script>
            document.write("OK");
        </script>
     
        <noscript>
            Sorry, JavaScript is not supported by your browser!
        </noscript>
       </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // let (path, set_path) = create_signal("".to_string());
    view! {

        <div id="maincontainer" class="bg-red" style="min-height: 100vh;" >

            // <section id="hero">
            //     <div class="justify-center" style="min-height: 100vh; align-items: center;">
            //         <img class="max-w-50 max-h-50" src="/assets/logo_w.png" alt="Liminal Journey" style="margin: 0;" />
            //     </div>
            // </section>

            // <section id="abstract">
            //     <div class="justify-center" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60 pb-80">
            //             <h2 class="apfelgrotezk text-white">ABSTRACT</h2>
            //             <div id="text-container-en">
            //                 <p>
            //                     <span class="weird">Anomaly</span>. <span class="weird">Irregularity</span>, <span class="weird">deviation</span> from the general rule, or from a structure, from a type considered normal.
            //                     Man is guided by a variety of factors, such as emotions, beliefs, religion, and social context. Factors like these can create barriers and hinder the ability to effectively learn from mistakes. In addition, changes require an often unfathomable sacrifice.
            //                 </p>
            //                 <p>
            //                     Misalignments, <span class="weird">deviations from the line</span>, are often sources of contamination and enrichment. <span class="weird">Anomalies</span> can challenge our preconceived ideas or traditional beliefs about reality. When we encounter <span class="weird">something unexpected</span> or that does not fit our mental patterns, we are forced to reconsider our beliefs and seek <span class="weird">alternative answers</span>. This process, through the acceptance of new ideas and creative flows, challenges the universally known and shakes the status quo. <span class="weird">Anomalies</span> can instill courage to face reality.
            //                 </p>
            //             </div>
                    
            //         </div>
            //     </div>
            // </section>
            
            <section id="haunting_loops">
                        
                
                
                <div class="justify-center w-100 bg-white hide-bar" style="min-height: 100vh; overflow-x: auto; white-space: nowrap;">
                    <div class="w-100" style="display: flex;">
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-80 max-h-50" src="/assets/media/anomalia/centoze/centonze_1.gif" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-80 max-h-50" src="/assets/media/anomalia/centoze/centonze_2.gif" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-80 max-h-50" src="/assets/media/anomalia/centoze/centonze_3.gif" alt="Liminal Journey" />
                        </div>
                    </div>
                </div>

                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Haunting Loops, </h1>
                        <h1>Margherita Centonze</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-red justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                                Exploration of the themes of fear and the unknown through a series of three pixel art-style GIFs. Each GIF features a girl as the protagonist, immersed in seemingly normal everyday situations that gradually reveal supernatural and frightening elements. The use of the GIF format creates a loop, enabling constant repetition of the content, which enhances the suspense as the events unfold repeatedly.
                            </p>
                        </div>
                    
                    </div>
                </div>
                
            </section>

            <section id="where_to">
                        
                <div class="justify-center w-100 bg-turchese text-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Where to??,</h1>
                        <h1>Morag Carver</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-turchese justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-black">
                            <p>
                                Where are we going? Why are we moving? How do we choose our destination? <br />
                                Between the moment where we ask ourselves these questions and the moment we take action to move,  lies the infinite number of uncertain directions that can lead to anomalies, mistakes, successes and disasters. <br />
                                EVERYTHING IS IN MOTION, NOTHING IS CERTAIN.
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="w-100 bg-turchese pb-40" style="min-height: 100vh;">
                        <img class="w-100" src="/assets/media/anomalia/morag/morag.jpg" alt="Liminal Journey" />
                </div>
                
            </section>

            <section id="untitled_giulia">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Untitled, </h1>
                        <h1>Giulia Coviello</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-red justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                                The three-dimensional sculpture, born from the interaction between artificial intelligence and the artists expression, explores the profound and delicate theme of suicide.
                                The fluid and sinuous forms of the work evoke a palpable sense of anguish and loneliness. The twisted and hunched position of the body and head adds an additional emotional dimension, powerfully conveying the complexity of the associated feelings.
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="justify-center w-100 bg-black" style="min-height: 100vh; overflow: hidden; white-space: nowrap;">
                    <div class="w-100" style="display: flex;">
                        <div class="justify-center w-100" style="flex: none; align-items: center;">
                            <img class="" src="/assets/media/anomalia/giulia_untitled/untitled.gif" alt="Liminal Journey" />
                        </div>
                    </div>
                </div>
                
            </section>

            <section id="traumaland">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Traumaland, </h1>
                        <h1>Giacomo Tazzini</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-red justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                                Traumaland is a non-place where tropical, rural and kitsch landscapes meet in a grotesque combination.
                                <br />
                                An amusement park thats not so amusing, where outcasts and oddballs wander through absurd and nightmarish scenarios.
                            </p>

                            <p>
                                These characters are always alone and in agony, seemingly uncapable of making sense of their existence, as their journey becomes more and more pointless.
                                <br /> 
                                They belong to no one and nothing, and maybe theyre desperately chasing the perspective of a better past, a long gone childhood.
                            </p>
                            <p>
                                The photos act as pieces of a narrative puzzle, like frames from a non-existing movie.
                            </p>
                            <p>
                                All photographs were captured on film, with very grainy stocks, through a Pentax K1000.
                            </p>
                            <p>
                                All locations were found along the Versilia coast, once a temple of Italian dolce vita, now a decadent and grotesque land.
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="justify-center w-100 bg-white hide-bar" style="min-height: 100vh; overflow-x: auto; white-space: nowrap;">
                    <div class="w-100" style="display: flex;">
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/1.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/2.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/3.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/4.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/5.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/6.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/7.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/8.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/9.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-ite
                        ms: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/10.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/11.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/12.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/13.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/14.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/15.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/16.jpg" alt="Liminal Journey" />
                        </div>
                    </div>
                </div>
                
            </section>

            <section id="michelle_hog">
                
                <div class="justify-center w-100 bg-blue text-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>hog;ripple; </h1>
                        <h1>Michelle Usai</h1>
                        <h1>2024</h1>
                    </div>
                </div>
                
                <div class="w-100 bg-white hide-bar h-100" style="overflow-x: auto; white-space: nowrap;">
                    <div class="w-100 h-100" style="display: flex;">
                        <div class="w-100 h-100" style="flex: none; align-items: center;">
                            <img class="h-100" src="/assets/media/anomalia/michelle_hog/hog_ripple.png" alt="Liminal Journey" />
                        </div>
                    </div>
                </div>
                
            </section>

            // <section id="ev_2ky">
                
            //     <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <h1>I don’t believe in coincidences, </h1>
            //             <h1>Rafael Bresciani</h1>
            //             <h1>2024</h1>
            //         </div>
            //     </div>

            //     <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <div class="text-white">
            //                 <p>
            //                     There is no chance <br />
            //                     There is no accident <br />
            //                     There is no exit <br />
            //                     Everything overloads <br />
            //                 </p>
            //             </div>
                    
            //         </div>
            //     </div>
                
            //     <div class="w-100 bg-white" style="min-height: 100vh;">
            //         <video autoplay loop muted controls class="w-100 h-100" src="/assets/media/anomalia/rafael/idbic_720.mov" alt="Liminal Journey"></video>
            //     </div>
                
            // </section>

            <section id="rafael_coincidences">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>"I don't believe in coincidences, "</h1>
                        <h1>Rafael Bresciani</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                                There is no chance <br />
                                There is no accident <br />
                                There is no exit <br />
                                Everything overloads <br />
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="w-100 bg-white" style="min-height: 100vh;">
                    <video autoplay loop muted controls class="w-100 h-100" src="/assets/media/anomalia/rafael/idbic_720.mov" alt="Liminal Journey"></video>
                </div>
                
            </section>

            // <section id="lorenzo_glitch_nav">
                
            //     <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <h1>Traumaland, </h1>
            //             <h1>Giacomo Tazzini</h1>
            //             <h1>2024</h1>
            //         </div>
            //     </div>

            //     <div class="bg-red justify-center" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <div class="text-white">
            //                 <p>
            //                     Traumaland is a non-place where tropical, rural and kitsch landscapes meet in a grotesque combination.
            //                     <br />
            //                     An amusement park thats not so amusing, where outcasts and oddballs wander through absurd and nightmarish scenarios.
            //                 </p>

            //                 <p>
            //                     These characters are always alone and in agony, seemingly uncapable of making sense of their existence, as their journey becomes more and more pointless.
            //                     <br /> 
            //                     They belong to no one and nothing, and maybe theyre desperately chasing the perspective of a better past, a long gone childhood.
            //                 </p>
            //                 <p>
            //                     The photos act as pieces of a narrative puzzle, like frames from a non-existing movie.
            //                 </p>
            //                 <p>
            //                     All photographs were captured on film, with very grainy stocks, through a Pentax K1000.
            //                 </p>
            //                 <p>
            //                     All locations were found along the Versilia coast, once a temple of Italian dolce vita, now a decadent and grotesque land.
            //                 </p>
            //             </div>
                    
            //         </div>
            //     </div>
                
            //     <div class="justify-center w-100 bg-white hide-bar" style="min-height: 100vh; overflow-x: auto; white-space: nowrap;">
            //         <div class="w-100" style="display: flex;">
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/1.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/2.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/3.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/4.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/5.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/6.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/7.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/8.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/9.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/10.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/11.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/12.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/12.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/13.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/14.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/15.jpg" alt="Liminal Journey" />
            //             </div>
            //             <div class="justify-center w-90" style="flex: none; align-items: center;">
            //                 <img class="max-w-90 max-h-90" src="/assets/media/anomalia/traumaland/16.jpg" alt="Liminal Journey" />
            //             </div>
            //         </div>
            //     </div>
                
            // </section>

            // <section id="john_youtube">
                
            //     <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <h1>I don’t believe in coincidences, </h1>
            //             <h1>Rafael Bresciani</h1>
            //             <h1>2024</h1>
            //         </div>
            //     </div>

            //     <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <div class="text-white">
            //                 <p>
            //                     There is no chance <br />
            //                     There is no accident <br />
            //                     There is no exit <br />
            //                     Everything overloads <br />
            //                 </p>
            //             </div>
                    
            //         </div>
            //     </div>
                
            //     <div class="w-100 bg-white" style="min-height: 100vh;">
            //         <video autoplay loop muted controls class="w-100 h-100" src="/assets/media/anomalia/rafael/idbic_720.mov" alt="Liminal Journey"></video>
            //     </div>
                
            // </section>

            <section id="oltre_realta_giulia">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Oltre la realtà, </h1>
                        <h1>Giulia Coviello</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                                In the digital era, photography and art have embraced artificial intelligence, opening the doors to new creative possibilities. AI-generated photographs, with their ability to manipulate and create a visual reality from a written prompt, can produce images that appear increasingly authentic, but are in fact the result of pure fiction and the imagination of the artist.
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="justify-center w-100 bg-black hide-bar" style="min-height: 100vh; overflow-x: auto; white-space: nowrap;">
                    <div class="w-100" style="display: flex;">
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-80 max-h-50" src="/assets/media/anomalia/giulia_untitled/oltre_la_realtà.png" alt="Liminal Journey" />
                        </div>
                    </div>
                </div>
                
            </section>

            <section id="letsglitch_destro">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Let It Glitch! Let It Glitch! Let It Glitch!, Svalbards islands, </h1>
                        <h1>Alessandro Destro</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                            "Wandering around Google Earth you can notice how the urban areas are extremely detailed and clean. As we move into areas that are wild and unwelcoming to humans, the system has difficulty creating images that are coherent and faithful to reality. All this is amplified in snowy areas, exalting errors beyond belief."</p>
                            <p>"“Let It Glitch! Let It Glitch! Let It Glitch!” is a digital journey that starts from Canada, passing through Greenland, Northern Europe, the Himalayan mountain range and then ending in Argentina, which highlights the errors that machines can create, and not the errors of human beings. This clip shows the various glitches present in the Svalbard Islands via Google Earth Studio."
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="w-100 bg-white" style="min-height: 100vh;">
                    <video autoplay loop muted controls class="w-100 h-100" src="/assets/media/anomalia/destro/destro_liminal.mp4" alt="Liminal Journey"></video>
                </div>
                
            </section>

            <section id="falling_federica">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Falling Kazuha, </h1>
                        <h1>kumicky</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                                Videogame glitch where the character Kazuha is trapped into an endless loop, repeatedly colliding into an invisible wall and while he is falling backward.
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="w-100 bg-black" style="min-height: 100vh;">
                    <video autoplay loop muted class="w-100 h-100" src="/assets/media/anomalia/kazuha/falling_kazuha.gif" alt="Liminal Journey"></video>
                </div>
                
            </section>

            <section id="untitled_saba">
                
                <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <h1>Un uomo che si considera normale, </h1>
                        <h1>Saba Rabie</h1>
                        <h1>2024</h1>
                    </div>
                </div>

                <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
                    <div class="w-60">
                        <div class="text-white">
                            <p>
                            "“In my country, Iran, where conformity is demanded and wearing the hijab is obligatory, anomaly arises when one is compelled to wear it despite not believing in it. The struggle to fit society&#39;s expectations forces individuals to wear a mask in public, concealing their true identity. Only within the confines of their homes can they be authentic. This has led to the creation of personal realities on the deceptive platform of social media, where everyone presents a carefully constructed facade. My artwork delves into this dichotomy, inviting reflection on the tension between conformity and self-expression in a society that demands conformity.”"
                            </p>
                        </div>
                    
                    </div>
                </div>
                
                <div class="justify-center w-100 bg-white hide-bar" style="min-height: 100vh; overflow-x: auto; white-space: nowrap;">
                    <div class="w-100" style="display: flex;">
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_1.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_2.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_3.gif" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_4.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center bg-red w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_5.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_6.jpg" alt="Liminal Journey" />
                        </div>
                        <div class="justify-center w-90" style="flex: none; align-items: center;">
                            <img class="max-w-90 max-h-90" src="/assets/media/anomalia/saba/saba_7.jpg" alt="Liminal Journey" />
                        </div>
                    </div>
                </div>
                
            </section>

            // <section id="glitch_gta">
                
            //     <div class="justify-center w-100 bg-white" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <h1>I don’t believe in coincidences, </h1>
            //             <h1>Rafael Bresciani</h1>
            //             <h1>2024</h1>
            //         </div>
            //     </div>

            //     <div class="bg-black justify-center" style="min-height: 100vh; align-items: center;">
            //         <div class="w-60">
            //             <div class="text-white">
            //                 <p>
            //                     There is no chance <br />
            //                     There is no accident <br />
            //                     There is no exit <br />
            //                     Everything overloads <br />
            //                 </p>
            //             </div>
                    
            //         </div>
            //     </div>
                
            //     <div class="w-100 bg-white" style="min-height: 100vh;">
            //         <video autoplay loop muted controls class="w-100 h-100" src="/assets/media/anomalia/rafael/idbic_720.mov" alt="Liminal Journey"></video>
            //     </div>
                
            // </section>
            

            <section id="guestbook">
                <div class="w-100 h-100 pt-80 pb-80">
                    <div class="justify-center">
                        <ChatComponent/>
                    </div>    
                </div>
            </section>

        </div>

        <footer class="pt-80 pb-40">
            <img src="/assets/logo_r.png" alt="Liminal Journey" style="max-width: 20%; max-height: 20%; margin: 0;" />
        </footer>

    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(GetChatMessages)]
pub async fn get_chat_messages(
    new_chat_message : (String, String)
) -> Result<Vec<(String, String, u64)>, ServerFnError> {
    logging::log!("Chat message received: {:?}", new_chat_message);
    // Filter the chat message for XSS
    let mut new_username = clean(&new_chat_message.0);
    let new_chat_message = clean(&new_chat_message.1);
    
    if new_username.clone().is_empty() {
        new_username = "Anonymous".to_string();
    }

    // Read chat.json, parse it, append the new chat message, and write it back to chat.json
    let base_path = std::env::current_dir()
    .map_err(|e| format!("Error getting current directory: {:?}", e)).unwrap();
    let chat_file_path = base_path.join("chat.json");

    // Read chat file to string, create it if it doesn't exist
    if !chat_file_path.exists() {
        let chat_file = std::fs::File::create(chat_file_path.clone())
            .map_err(|e| format!("Error creating chat file: {:?}", e)).unwrap();
        chat_file.sync_all()
            .map_err(|e| format!("Error syncing chat file: {:?}", e)).unwrap();
    }

    let chat_file = std::fs::read_to_string(chat_file_path.clone())
        .map_err(|e| format!("Error reading chat file: {:?}", e)).unwrap();

    let mut chat_messages : Vec<(String, String, u64)> = Vec::new();
    // If chat file is not empty, parse it
    if !chat_file.is_empty() {
        chat_messages = serde_json::from_str(&chat_file)
        .map_err(|e| format!("Error parsing chat file: {:?}", e)).unwrap();
    }

    // Append the new chat message
    if new_chat_message.len() > 0 && new_chat_message.len() < 1000 {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        chat_messages.push((new_username.clone(), new_chat_message.clone(), timestamp));
        // Write the chat messages back to chat.json
        let chat_file = std::fs::File::create(chat_file_path)
            .map_err(|e| format!("Error creating chat file: {:?}", e)).unwrap();
        serde_json::to_writer(chat_file, &chat_messages)
            .map_err(|e| format!("Error writing chat file: {:?}", e)).unwrap();
    }
    // Only return the last 5 chat messages
    let chat_messages = chat_messages.iter().rev().take(5).cloned().rev().collect();

    Ok(chat_messages)
}

#[component]
pub fn ChatComponent() -> impl IntoView {
    let (chat, send_chat) = create_signal(("".to_string(), "".to_string()));
    let chat_input_ref: NodeRef<Input> = create_node_ref();
    let name_input_ref: NodeRef<Input> = create_node_ref();
    let inc = create_action(|_: &()| adjust_message_count(1, "test".into()));

    let on_submit = move |ev: SubmitEvent| {
        // Prevent the page from refreshing
        ev.prevent_default();
        // Get a reference to the chat text input box
        let new_chat_message = chat_input_ref().expect("<input> does not exist").value();
        let new_username = name_input_ref().expect("<input> does not exist").value();
        // Send the chat message to the server
        send_chat((new_username.to_string(), new_chat_message.to_string()));
        // Clear text input box
        chat_input_ref().expect("<input> does not exist").set_value("");
        inc.dispatch(());
    };

    // our resource
    let chat_messages = create_local_resource(
        chat,
        // every time `chat` changes, this will run
        |new_chat_message| async move {
            logging::log!("Chat by {}: {}", new_chat_message.0, new_chat_message.1);
            get_chat_messages(new_chat_message).await
        },
    );

    #[cfg(not(feature = "ssr"))]
    let message_count_value = {
        use futures::StreamExt;
        let mut source =
            gloo_net::eventsource::futures::EventSource::new("/ws")
                .expect("couldn't connect to SSE stream");
        let s = create_signal_from_stream(
                source
                .subscribe("message")
                .unwrap()
                .map(|value| match value {
                    Ok(value) => value
                        .1
                        .data()
                        .as_string()
                        .expect("expected string value"),
                    Err(_) => "0".to_string(),
                }),
        );
        on_cleanup(move || source.close());
        s
    };

    #[cfg(feature = "ssr")]
    let (message_count_value, _) = create_signal(None::<i32>);

    // If there's a new message count value sent from the server, initiate a GET for new chat messages by sending an empty message.
    // This could of course be done more efficiently by directly fetching the new chat message from the server.
    create_effect(move |_| {
        let count = message_count_value.get().unwrap_or_default();
        send_chat((count.to_string(), "".to_string())); // If count is not used anywhere, the effect will never be triggered.
    });

    view! {
        <div class="">
            <h2 class="">Guestbook</h2>
            
            <div class="max-w-60">
                {
                    move || { 
                        match chat_messages.get() {
                            Some(result) => {
                                match result {
                                    Ok(messages) => {
                                        messages.into_iter()
                                        .map(move |n| {
                                            let (user, message, _timestamp) = n.clone();
                                            view!{
                                            <div>
                                                <div>
                                                    <p><span class="text-white">{user}: </span> {message}</p>
                                                </div>
                                            </div>
                                            }
                                        }).collect_view()
                                    },
                                    Err(e) => {
                                        logging::log!("Error displaying Guestbook: {:?}", e);
                                        leptos::View::Text(view! {
                                            "ERROR: Could not display Guestbook. Please try again later."
                                        })
                                    }
                                }
                            },
                            None => {
                                leptos::View::Text(view! {
                                    "No messages left"
                                })
                            }
                        }
                    }
                }

            </div>

            <div>
                <form on:submit=on_submit class="w-100 pt-40 pb-40">
                    <div class="justify-center">
                       <div class="max-w-60">
                        <input class=" mb-12" type="text" placeholder="Name" node_ref=name_input_ref />
                        <br />
                        <input class=" mb-12" type="text" placeholder="Leave" node_ref=chat_input_ref />
                        <br />
                        <button class="w-100 mb-12 bg-black text-red" type="submit" id="button-send">Add</button>
                       </div>
                    </div>
                </form>
            </div>

        </div>

    }
}

#[cfg(feature = "ssr")]
pub mod ssr_imports {
    pub use broadcaster::BroadcastChannel;
    pub use once_cell::sync::OnceCell;
    pub use std::sync::atomic::{AtomicI32, Ordering};

    pub static COUNT: AtomicI32 = AtomicI32::new(0);

    lazy_static::lazy_static! {
        pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
    }

    static LOG_INIT: OnceCell<()> = OnceCell::new();

    pub fn init_logging() {
        LOG_INIT.get_or_init(|| {
            simple_logger::SimpleLogger::new().env().init().unwrap();
        });
    }
}

#[server]
pub async fn get_message_count() -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    Ok(COUNT.load(Ordering::Relaxed))
}

#[server]
pub async fn adjust_message_count(
    delta: i32,
    msg: String,
) -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {:?}", msg);
    Ok(new)
}
