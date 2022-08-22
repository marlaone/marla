# My love to rust and go

## Introduction

For many years I was mainly a PHP backend and JS frontend developer until I discovered Go a few years ago.
Go opened up a whole new world for me - I was always against all these type based languages at first. "That's all unnecessarily complicated, if you program well, PHP is also stable and bug-free", I always thought to myself.
But Go is also so easy to program AND offers just a beautiful type safety and above all - Go is not interpreted. Asynchronous programming was suddenly much easier.

Many side projects emerged from this first to get to know the programming language.

A tournament tool, through which we could manage our company's internal FIFA tournaments.

[Picture of the system]

Frames captured from a certain Windows window (Blobby Volley) and with OpenCV the blobs were detected in there, as well as the ball. Based on this information, I wrote a bot that was controlled by a virtual Xbox controller.

[Picture of the Blobby Volley with debug view]

And from then on Go just started to become really cool for me. You can just solve all kinds of problems with it. No matter if you want to control a standard HTTP server or Windows services, machine learning, etc. It feels like anything is possible. In PHP such sideprojects would have been unthinkable for me.

So directly to the next side project: Why not program a 2D game in Go?

[Picture of the game]

And so I developed a great love for Go.

--Side Note: Yes, that's a lot of side projects and none of them made it to a final product. But that's another topic for a later lesson.

## The Great Love For Go

From then on, all my projects were in Go and to this day, I would also like to get the JavaScript in the frontend replaced with Go so I can just play with the cool kids all the time. There are possibilities, but it's not mature enough for big, productive projects yet.

Meanwhile the love was so big that my mother even crocheted me the Gopher, which now always accompanies me on my desk.

[Image by Gophy]

After I had annoyed my colleagues long enough and had shown with a proof-of-concept how much easier, more stable and performant we could develop in Go, the programming language quickly found its way into my daily work. We were at a point where we wanted to redevelop our entire product anyway, so the timing was perfect for Go.

Our tracking was then also converted from PHP to Go and instead of 80,000 requests per minute, we now manage 72,000,000 requests per minute. Of course, Go alone does not make this difference, but the fact that we can now work much more asynchronously instead of being stuck in an interpreted programming language allows us to implement completely different architectures.

And we're also doing something good for the climate. After we went live with the new tracking at a customer, the CPU cooled down from an average of 80°C to 40°C - so much lower energy consumption. Theoretically, we wouldn't need any more servers with super powerful hardware and could save money, but the topic is even more complex and that's not the point here.

[Picture of the CPU]

Captivated by the new performance, I went on to think: "How can this be optimized even more?" and after playing through the "Go" game, I discovered Rust.

## The Exotic New Woman

Unsuspecting and actually in a happy relationship with Go, Rust came around the corner. Supposedly the fastest programming language after C (or even just as fast) that you can currently learn. And even without the memory headaches of C.

That sounded interesting. So I built a little side project in Rust. Our trainee had participated in a national competition for computer science and there was an interesting task, where you had to develop an algorithm, which fulfills certain conditions and finds the best route from start to finish.
I had already built the whole thing in Go and the program needed 3 seconds for the largest example data set.

After I almost had an existential crisis because of Rust's compiler and its ownership system, after a few days I actually ran the program and it was BLAZINGLY... slow... Why? Rust is supposed to be faster, right? After eternal pprof analyzing I always ended up with the HashMap of Rust. But how can a map be so slow? In Go it’sfast? After eternal googling, I found two opinions about HashMaps. "HashMaps are fast" and "HashMaps are slow". Perfect and now? Not really understanding the whole thing yet, I just replaced all HashMaps with Vecs and suddenly I was much faster. Why? As far as I understood, Go Maps work internally exactly the same way. For small "maps", the maps are actually arrays and only become "HashMaps" for large "maps". It is just faster to search a complete, small array for a key, than to solve this via a HashMap.
So I guess that was the approach I had to take in Rust and stayed with the Vecs.

--Warning: The thing with the maps is only a theory. Who would like to know it more exactly, can inform itself naturally over uncle Google correctly.

But still the Rust program was slower than my Go program. Go at 3 seconds, Rust at 10 seconds. That is stupid.
I wrote `cargo build --release` and executed the binary. Suddenly Rust needed only 1.2 seconds. So faster than Go! Because of Go I didn't know that there is a difference in performance between `go run` and `go build`.
So I learned something again. Now Rust had me! It is faster than Go! And after getting over the steep learning curve of Rust, the programming language was actually really fun. The syntax was totally appealing. Practical functions like `.map()`, `.from()`, `.unwrap_or()`, `.unwrap_or_default()` totally fell into place. `Result<T,E>` and `Option<T>` are totally nice enums. Oh and man, ENUMS! How awesome are those in Rust, please? But I'm digressing again.

So on to a new project. A product of ours that loads, analyzes and aggregates millions of records from a MongoDB live would be a great example project. Especially because it's really not fast in PHP.

No sooner said than done - COMPILE ERRORS. Weeks I despaired on async in Rust. It never worked like I wanted. Especially because "async" exists only half in Rust. For that you need an extra runtime like `Tokio`.
Eventually it worked somehow, but the code was horrible and it wasn`t fast either. And thanks to a mutex, at some point my HTTP requests were blocked. It was hell.
So I gave up on Rust and switched back to Go.

I built the product in my old, great love Go again. And it went so fast, so easy. Everything was as beautiful as before.

But again and again I had to think of my adventures with Rust. It was so wild and you felt so challenged. It wasn't all so easy, but when it worked out, the satisfaction was all the greater. Wait a minute. Somehow I'm digressing into an adult novel right now.

I then dared to work on the Rust version of the product again for a few weeks. And at some point it *clicked* with the async and the project ran stable. And above all performant. That was again a very nice moment.

And so it went on in my life. During the day I spent my time with Go and in the evenings I was always busy with new challenges in Rust.

## Do I Live In A Polygamous Relationship?

What I could never imagine in my private life is a lot more difficult in programming. I love Go, Go has made me such a much better programmer. I have learned so many new things in a short period of time. However, Go has also led me to Rust and Rust is just an exciting adventure that still makes my programmer heartbeat a little faster than Go.

### What Would Happen If I Were Only Allowed To Program Monogamous?

The decision would be easy for me. As much as Rust always finds its place before Go in sideprojects, my decision would immediately be "Go". It depends on the use case you're programming in, of course. However, since I mainly build web backends, Go is simply the best match there.
I find it so incredibly easy to set up a project in Go. No matter if it's a web backend or anything else. It's also much easier to get started in Go than in Rust. So even from an employer's point of view, it's much easier to teach a programmer Go than Rust. I would say that I can probably build standard web projects in Rust and Go equally fast, because there are great open source projects for both programming languages, but Go delivers the best developer experience.

## Conclusion *TRIGGER WARNING*.

Both programming languages will continue to have a place in my heart, but Go remains my great love. Go has made me a real man.... I mean programmer. Showed me whole new ways of thinking and brought new fun to programming in my spare time. This website is still built in Rust because the exotic adventure appealed to me too much, but I would always prefer Go if you hold a gun to my chest.

Still, I also think it's good to think outside the box when it comes to programming and learn new programming languages. This way you learn a lot about structures, data types and performance. Why is one better than the other? The journey with Go and Rust has made me a much better programmer because I now understand programming and a programming language better and can find my way around problems faster.

Final triggering words: On the backend, I see no raison d'être for programming languages other than Rust and Go. Rust does the best low level tasks and Go does the best high level tasks. Nobody needs C or Java anymore. We don't even need to start talking about those "programming languages" Python and JavaScript. And were there any other programming languages at all?

And after I successfully triggered over 90% of IT people with this, I wish you a nice day anyway!
