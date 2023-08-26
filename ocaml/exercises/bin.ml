(* Write a function last : 'a list -> 'a option that returns the last element of a list *)

(* # last ["a" ; "b" ; "c" ; "d"];;
- : string option = Some "d"
# last [];;
- : 'a option = None *)

let rec last = function
  | [] -> None
  | [x] -> Some x
  | _ :: t -> last t

(* Find the last but one (last and penultimate) elements of a list *)

(* # last_two [ "a" ; "b" ; "c" ; "d" ];; *)
(* - : (string * string) option = Some ("c", "d") *)
(* # last_two [ "a" ];; *)
(* - : (string * string) option = None *)

let rec last_two = function
  | [] | [_] -> None
  | [x; y] -> Some (x, y)
  | _ :: t -> last_two t



(* Find the k'th element of a list *)

(* # at 3 [ "a" ; "b"; "c"; "d"; "e" ];; *)
(* - : string option = Some "c" *)
(* # at 3 [ "a" ];; *)
(* - : string option = None *)

let () =
    print_string "at 3 [ \"a\" ; \"b\"; \"c\"; \"d\"; \"e\" ];;\n";
