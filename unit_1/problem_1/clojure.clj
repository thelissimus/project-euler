(defn solution [n]
  (->> n
       (range)
       (filter #(or (zero? (mod % 3))
                    (zero? (mod % 5))))
       (reduce +)))

(def result (solution 1000))
