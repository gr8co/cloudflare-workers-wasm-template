import quotes from 'quotesy';

export function quote() {
    return quotes.random().text;
}
