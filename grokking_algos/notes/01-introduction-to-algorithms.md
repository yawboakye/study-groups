> binary search is an algorithm; its input is a sorted list of elements (i’ll
> explain later why it needs to be sorted). if an element you’re looking for is
> in that list, binary search returns the position where it’s located.
> otherwise, binary search returns `null`.

technically, what binary search returns in case of a miss should be dependent on
the implementation. if programming language is rust, it might be more elegant to
return an `Option<T>`. in programming languages without the optional type
facility, there might be other ways to avoid `null`, e.g. returning (or
throwing) and exception.
