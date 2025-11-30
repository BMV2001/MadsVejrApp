let temp = document.getElementById("temp")
let coord = document.getElementById("coord")
let glink = document.getElementById("glink")

async function getWeatherData(){
    let weatherData = await
        fetch("fetchWeather/",
        {
        method: "GET",
        headers: {
                "Content-Type": "application/json"
            }
        })
    if (weatherData.status == 200){
        let JsonWeather = await weatherData.json()
        let coordSet = `${JsonWeather.coord[0]},${JsonWeather.coord[1]}`

        temp.innerText = `Det er altså ${JsonWeather.temp}C`
        coord.innerText = `på koordinatsættet ${coordSet}`
        glink.href = `https://www.google.com/maps/place/${coordSet}`
        glink.innerText = "(KLIK HER :))"
    }
    else {
        document.body.innerHTML =
            "<h1> >:/ Det der JSON kald virkede ikke helt</h1>" +
            "<Button onclick=getWeatherData()>Men prøv lige igen ;)</Button>"
    }
}

getWeatherData()
