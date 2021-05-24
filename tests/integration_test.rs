use bfc::{interpreter};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_forward() {
        let mut m_pointer = 0;
        let mut mem: [u8;5] = [0;5];
        let size = 5;
        interpreter(String::from(">"), &mut mem, &mut m_pointer, size);
        assert_eq!(m_pointer, 1);
    }
    #[test]
    fn test_backward() {
        let mut m_pointer = 0;
        let mut mem: [u8;5] = [0;5];
        let size = 5;
        interpreter(String::from("<"), &mut mem, &mut m_pointer, size);
        assert_eq!(m_pointer, size-1);
    }

    #[test]
    fn test_increment() {
        let mut m_pointer = 0;
        let mut mem: [u8;5] = [0;5];
        let size = 5;
        interpreter(String::from("+"), &mut mem, &mut m_pointer, size);
        assert_eq!(mem[m_pointer], 1);
    }

    #[test]
    fn test_decrement() {
        let mut m_pointer = 0;
        let mut mem: [u8;5] = [0;5];
        let size = 5;
        interpreter(String::from("-"), &mut mem, &mut m_pointer, size);
        assert_eq!(mem[m_pointer], 255);
    }

    #[test]
    fn test_loop() {
        let mut m_pointer = 0;
        let mut mem: [u8;5] = [0;5];
        let size = 5;
        interpreter(String::from("+++[>+<-]"), &mut mem, &mut m_pointer, size);
        assert_eq!(mem[0], 0);
        assert_eq!(mem[1], 3);
    }

}