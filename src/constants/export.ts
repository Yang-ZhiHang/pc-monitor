import { ExportFormat, ExportOption } from "@/types/export";


export const exportOptions: Record<string, ExportOption> = {
    [ExportFormat.HTML]: {
        title: "export.export-fmt.html.title",
        description: "export.export-fmt.html.desc",
        icon: "fa-brands fa-html5 text-orange-500 text-4xl"
    },
    [ExportFormat.CSV]: {
        title: "export.export-fmt.csv.title",
        description: "export.export-fmt.csv.desc",
        icon: "fa fa-file-text text-green-500 text-4xl"
    },
    [ExportFormat.JSON]: {
        title: "export.export-fmt.json.title",
        description: "export.export-fmt.json.desc",
        icon: "fa fa-code text-purple-500 text-4xl"
    }
}