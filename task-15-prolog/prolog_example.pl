
% facts
human(benjamin).
asse(daniel).
friend(benjamin, daniel).
friend(daniel, matteus).


student(Person):-
    human(Person),
    \+ asse(Person).

% functions, also called rules or predicates
friend_of_a_friend(A, B):-
    friend(A, X),
    friend(X, B).

% is true if Element is the last element of the list
last([H], Element):-
    Element = H.
last([H|T], Element):-
    last(T, Element).

% Is true if Element is the Nth element of the list
% 1-indexing
nth(1, [Element], Element).
nth(1, [H|T], Element):-
    Element = H.
nth(N, [H|T], Element):-
    N1 is N - 1,
    nth(N1, T, Element).

% True if Reversed is the reversed list of the other list
reverse_naive([], []).
reverse_naive([Head|Tail1], Reversed) :-
    reverse_naive(Tail1, Tail2),
    append(Tail2, [Head], Reversed).

% Same as reverse_naive but implemented with an accumulator
% Helper: calls with an empty accumulator
reverse_acc(List, Tail) :-
    reverse_acc(List, [], Tail).

reverse_acc([], Reversed, Reversed).
reverse_acc([Head|Tail], Acc, Reversed) :-
    reverse_acc(Tail, [Head|Acc], Reversed).