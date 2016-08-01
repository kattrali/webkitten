function description()
  return "Smart defaults command for opening a website or doing a search"
end

-- Opens anything that looks like a URL directly, or falls back to a web search
-- using the configuration option `general.search-engine-url` to construct the
-- query. The search engine option defaults to DuckDuckGo.
function run()
  local query = table.concat(arguments, " ")
  if query:match("^([^?][%w%p]+%.[%w%p]+)$") then
    open_uri(query)
  else
    local engine = lookup_string(config_file_path, "general.search-engine-url")
    if #engine == 0 then
      engine = "https://duckduckgo.com"
    end
    open_uri(table.concat({engine, "?q=", url_encode(query)}, ""))
  end
  return true
end

function open_uri(target)
  local windex = focused_window_index()
  if windex ~= NOT_FOUND then
    load_uri(windex, focused_webview_index(windex), target)
  else
    open_window(target)
  end
end

function url_encode(str)
  if (str) then
  str = string.gsub (str, "\n", "\r\n")
  str = string.gsub (str, "([^%w %-%_%.%~])",
    function (c) return string.format ("%%%02X", string.byte(c)) end)
  str = string.gsub (str, " ", "+")
  end
  return str
end
