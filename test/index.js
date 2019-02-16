const fs =  require('fs')
const childProcess = require('child_process')
const express  = require('express')


const app = express()
var server = require('http').createServer(app);



app.get('/',(req, res) => {

  const take = {
    headers: req.headers,
    body: req.body,
    query: req.query,
    params: req.params,
  }

fs.readFile("config.json", ((err, file) => {

  if (err) {
    throw new Err("On ne trouve pas le fichier")
  }

  const datas = JSON.parse(file)

  datas.conf.forEach(data => {

    const process = childProcess.spawn(`${data.language}`, [`${data.path}`, JSON.stringify(take)] );

    // const process = childProcess.fork(data.path, [data]) // TODO CHANGE TO TRUE || false


    process.stdout.on('data', (data) => {
      console.log(`child stdout:\n${data}`)
      // return   res.json({message: "msg"})
    })
    process.stdout.on('return', (data) => {
      console.log(`RETURN ==>:\n${data}`)
      // return   res.json({message: "msg"})
    })

    // process.on('exit', (toto, tata) => {
    //   console.log(toto, tata);
    // })

  })
  return


}))
})


 server.listen(5000, function(){
   console.log('listening on 5000');
 });
