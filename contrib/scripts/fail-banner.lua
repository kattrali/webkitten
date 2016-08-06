function description()
  return "Shows a banner with an error message when a page fails to load"
end

function on_fail_uri()
  run_javascript(window_index, webview_index, string.format([[
    var element = document.createElement("div");
    element.style.position = "fixed";
    element.style.top = 0;
    element.style.left = 0;
    element.style.zIndex = 9999;
    element.style.background = "red";
    element.style.width = "100%%";
    element.style.color = "white";
    element.onclick = function () { this.parentElement.removeChild(this) };
    var message = document.createTextNode("Failed to load %s : %s Click to dismiss.");
    element.appendChild(message);
    document.body.appendChild(element);
  ]], requested_uri, clean_message(error_message)))
end

function clean_message(message)
   return message:
    gsub("&","&amp;"):
    gsub("<","&lt;"):
    gsub(">", "&gt;"):
    gsub('"', '&quot;'):
    gsub("'", '&#39;'):
    gsub("/", '&#x2F;')
end
