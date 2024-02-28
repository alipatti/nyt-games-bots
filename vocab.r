library(tidyverse)

VOCAB_SIZE <- 1000
WORD_LIST_URL <- "https://raw.githubusercontent.com/hackerb9/gwordlist/master/frequency-alpha-alldicts.txt"

df <- read_table(WORD_LIST_URL) %>%
        select(-`#RANKING`) %>%
        mutate(across(c(PERCENT), parse_number))

words <- df %>%
        filter(str_length(WORD) >= 3) %>%
        head(VOCAB_SIZE) %>%
        .$WORD

write_lines(words, "word_list.txt")
