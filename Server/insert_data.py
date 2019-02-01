import csv
import mysql.connector

cnx = mysql.connector.connect(user='root',
                              host='127.0.0.1',
                              database='plums')

mycursor = cnx.cursor(buffered=True)

themes_added = 0
topics_added = 0

with open('data.csv') as csv_file:
    csv_reader = csv.reader(csv_file, delimiter=',')
    for row in csv_reader:
        mycursor.execute("SELECT name FROM themes WHERE name = %s;", (row[0],))
        result = mycursor.fetchone()
        if result == None:
            mycursor.execute("INSERT INTO `themes` (name) VALUES (%s)", (row[0],))
            cnx.commit()
            themes_added += 1
        mycursor.execute("SELECT id FROM `themes` WHERE name=%s", (row[0],))
        theme_id = mycursor.fetchone()[0]

        for topic in row[1:]:
            mycursor.execute("SELECT name FROM `topics` WHERE name= %s", (topic,))
            result = mycursor.fetchone()
            if result == None:
                mycursor.execute("INSERT INTO `topics` (name, theme) VALUES (%s,%s)", (topic, theme_id,))
                cnx.commit()
                topics_added += 1

print("Themes added: " + str(themes_added))
print("Topics added: " + str(topics_added))
cnx.close()
