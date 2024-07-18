/*   Below is the interface for Iterator, which is already defined for you.
 *
 *   type Iterator struct {
 *       
 *   }
 *
 *   func (this *Iterator) hasNext() bool {
 *		// Returns true if the iteration has more elements.
 *   }
 *
 *   func (this *Iterator) next() int {
 *		// Returns the next element in the iteration.
 *   }
 */

type PeekingIterator struct {
    iterator *Iterator
    peekVal int
    hasNextVal bool
}

func Constructor(iter *Iterator) *PeekingIterator {
    peekVal := iter.next()
    peekiter := PeekingIterator {
        iterator: iter,
        peekVal: peekVal,
        hasNextVal: true,
    }
    return &peekiter
}

func (this *PeekingIterator) hasNext() bool {
    return this.hasNextVal
}

func (this *PeekingIterator) next() int {
    returnValue := this.peekVal
    this.hasNextVal = this.iterator.hasNext()
    this.peekVal = this.iterator.next()
    return returnValue
}

func (this *PeekingIterator) peek() int {
    return this.peekVal
}


