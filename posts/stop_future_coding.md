# Stop Future Coding

All too often I see instances where someone (myself included) begins designing
systems for requirements that don't exist. This is something I term as *future
coding*. In other words, building software for the "what if".

This is a practice that I think is all too common in software, and I continue to
observe how it leads to systems which become overly bloated, more complex and
ultimately degrade over time.

Instead, we should be looking 

## The wrong abstractions

Future coding can lead to creating abstractions within our code which serves no
purpose well, and many purposes poorly. 

Abstractions created for the purpose of being resilient to change almost always
begin to leak over time. When the future requirements they were written to
handle never materialise, the abstraction hangs around and is seldom refactored.

You've probably come across a generic function in a program which is called with
a total of only two separate types, with those types having limited functional
overlap. This becomes more evident if such a function contains a greater than
normal number of assertions for example.

## Why we do it?

Most future coding is done with good intentions. The intention here clearly
isn't the problem. Building software which is resilient to change is very much a
good thing to strive for. Done well, it can make or break a product's survival
after launch.

Most future coding is a result of the developer attempting to preempt future
requirements and attempting to build something they believe will be better
suited to this change.

> This preemption leads to software containing abstractions which will begin to
> leak over time.

## How you can identify it

In order to prevent your software becoming unnecessarily complex, a level of
pragmatism 

## What to do instead


---

Notes:

Abstractions are costly, especially the wrong abstractions. They also have a
nasty habit of growing over time. Abstractions are also a frequent result of
developers attempting to predict how software will change over time.

Future coding is a phenomena which arises when building software to a
specification which the developer thinks is likely to arise in the future.
Certain abstractions are made ahead of time, certain things are broken up ahead
of time, all to proactively minimise change in the future.

This is fundamentally flawed. Requirements cannot be predicted. Worse, most
teams do not take log of the decisions that were made in order to support these
future requirements, and when they don't materialise, undo such decisions. 
