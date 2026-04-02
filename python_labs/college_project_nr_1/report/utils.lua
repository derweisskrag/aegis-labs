local M = {}

function M.render_table(data_object)
    local caption = data_object.caption or "Data Table"
    local rows = data_object.rows
    local first = rows[1]

    -- Extract column names dynamically
    local columns = {}
    for key, _ in pairs(first) do
        table.insert(columns, key)
    end

    -- Sort columns for consistent order (optional)
    table.sort(columns)

    -- Build LaTeX
    local latex = "\\begin{table}[h]\n\\caption{" .. caption .. "}\n"

    -- Build column spec: one 'c' per column
    latex = latex .. "\\begin{tabular}{|" .. string.rep("c|", #columns) .. "}\n\\hline\n"

    -- Header row
    for i, col in ipairs(columns) do
        latex = latex .. "\\textbf{" .. col:gsub("_", "\\_") .. "}"
        if i < #columns then
            latex = latex .. " & "
        end
    end
    latex = latex .. " \\\\ \\hline\n"

    -- Data rows
    for _, row in ipairs(rows) do
        for i, col in ipairs(columns) do
            latex = latex .. tostring(row[col] or "")
            if i < #columns then
                latex = latex .. " & "
            end
        end
        latex = latex .. " \\\\ \\hline\n"
    end

    latex = latex .. "\\end{tabular}\n\\end{table}"
    return latex
end



function M.render_figure(diagram_obj)
    local path = diagram_obj.path
    local caption_text = diagram_obj.caption 
    
    -- Figure remains centered, but caption will be left-aligned via YAML setting
    local latex = "\\begin{figure}[ht]\n\\centering\n"
    latex = latex .. "\\includegraphics[width=0.8\\textwidth]{" .. path .. "}\n"
    latex = latex .. "\\caption{" .. caption_text .. "}\n"
    latex = latex .. "\\end{figure}"
    
    return latex
end

return M