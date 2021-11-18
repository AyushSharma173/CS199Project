# CS199Project
Group Name: Dynamic Dudes
Group Members: Ayush Sharma(ayushs5), Aarnav Chaturvedi(aarnavc2), Vibhav Rajkumar (vibhavr2)

Project Introduction:

Please provide a brief description of your project. List the goals and objectives of your project and explain why you’ve chosen to work on this project.

We intend to create a differential equation solver using Rust. The goal would be to create an application that can parse basic mathemtical functions, and use that as a model for an n^th order differential equation, and numerically solve that equation over a specific time period. The reason we chose this project is because differential equations are an extremely important field of mathematics in engineering and physics, and in a lot of cases, cannot be solved analytically. In these cases, numerical methods are useful, and that is what the project seeks to accomplish. 

System Overview

Please provide a moderate-length technical description of the major components of your project. This should also function as a sort of ‘roadmap’ for tasks you need to complete for your project to be functional.

There are 3 major steps in this project: Building an application which creates a parse tree from mathematical expressions, building an application that can use a parse tree to generate an executable rust function, and creating a numerical integrator which leverages parallelization. 

The first step involves taking a string input from the user, and parsing it to create a parse tree. This would be similar to wolfram alpha, but in a significantly more simlified manner. The rules behind whether a string is a valid mathematical expression or not will have to be a lot stricter than in the conventional sense.

Possible Challenges
Please list some of the challenges you foresee running into.
We envision that implementing a parse tree in order to parse mathematical expressions will be the most difficult part of this project. 
If you’re basing the project off of some other work or if you received inspiration from an existing project, please list it here!
