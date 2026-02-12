# DD1338 Week 18

Author: Dmitry Chirin
(Heavily inspired by ML-assignment done by Anton Lilja and Morris Hansing, ++22 ❤️)

## ML - Machine Learning 🤖
AI is all over the place nowadays. And as some wise person once said: "If you can't beat them, join them!" 😈

This assignment is about making a Neural Network (NN), one of the models in Machine Learning.

## Prepare for your assignment

1) Create a repository named `<KTH_ID>-ml`.
2) Get started! (Probably with Python, but it can be done in any language. 
Matrix multiplications are the same anywhere anyway 🤷‍♂️)

## Assignment

For this week you have to do the following:
- **Implement some sort NN for the MNIST data set**
  - The baseline is to get 80% of the unseen data right (actually quite simple)
  - Can be done with any algorithm
    - Could be as simple as some kind of [genetic algorithm](https://medium.com/towards-data-science/gas-and-nns-6a41f1e8146d) (natural selection)
    - [SGD](https://mohitmishra786687.medium.com/stochastic-gradient-descent-a-basic-explanation-cbddc63f08e0) could be a good goal
    - As advanced as you would like (back propagation, etc.)
  - Most of the implementation is supposed to be done by you from scratch.
    - You can use libraries for some math, such as vectors, but try to implement matrix multiplication by yourselves
    (if it seems to be too hard, you could use a library, but that's as much as is allowed).
      - Examples of acceptable libraries in Python are: numpy, pandas, scipy, matplotlib, numba
    - You are not allowed to use TensorFlow, Pytorch, or other libraries which implement the whole thing, 
    or large parts of it
- (Optional) Optimize your neural network more. Try implementing one of the following:
  - Experimenting more with the amount of nodes you have in each layer, how many layers, etc.
  - Testing [different activation functions](https://www.geeksforgeeks.org/activation-functions-neural-networks/) (ReLU, Sigmoid, tanh, softmax, softplus)
  - [Dropout layers](https://towardsdatascience.com/dropout-in-neural-networks-47a162d621d9/)
  - [Adaptable (lowering) learning rate](https://www.jeremyjordan.me/nn-learning-rate/)
  - [Early stopping](https://en.wikipedia.org/wiki/Early_stopping) (preventing overfitting)
  - Implementing some sort of [gradient decent](https://bhatnagar91.medium.com/how-neural-networks-learn-using-gradient-descent-f48c2e4079a6)
  - [Momentum](https://machinelearningmastery.com/gradient-descent-with-momentum-from-scratch/)
  - [Back propagation](https://www.geeksforgeeks.org/feedforward-neural-network/)

### Reference values

| Result | How good is that?                              |
|--------|------------------------------------------------|
| 70-85% | Genetic algorithm                              | 
| 90%    | Simple SGD                                     | 
| 98%    | Really good, Gradient Descent, Momentum, etc.  | 
| 99.7%  | World class result (mostly as good as it gets) |

### Java alternative

For this week, the java alternative is task-15, because you've already done graphs and trees in the previous assignment.

### The MNIST data set

You can get the MNIST data set from one of the following sources:
- [MNIST for rust](https://docs.rs/mnist/latest/mnist/)  
- [MNIST for python](https://pypi.org/project/python-mnist/)  
- [Raw data](http://yann.lecun.com/exdb/mnist/)

## Good resources

- ✨Best ✨: [3b1b YT series](https://www.youtube.com/watch?v=aircAruvnKk&list=PLZHQObOWTQDNU6R1_67000Dx_ZCJB-3pi)
- [NN and DL book](http://neuralnetworksanddeeplearning.com/index.html)
- Another style of [YT series](https://www.youtube.com/playlist?list=PLgcwDw9tMf6gmk0kTvAE0FlZs1S09zJnE)
- [Hacker's guide to Neural Networks](https://karpathy.github.io/neuralnets/) (blog, sounds cool 😎)
