function description()
  return "Hide results from 'find'"
end


function run()
  windex = focused_window_index()
  hide_find(windex, focused_webview_index(windex))
  return true
end
