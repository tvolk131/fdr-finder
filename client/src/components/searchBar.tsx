import {
  IconButton,
  InputBase,
  AccordionSummary,
  Accordion,
  AccordionDetails,
  Divider,
  Chip,
  CircularProgress,
  TextField,
  Slider
} from '@mui/material';
import {Autocomplete} from '@mui/lab';
import {Theme, styled} from '@mui/material/styles';
import {createStyles, makeStyles} from '@mui/styles';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import CloseIcon from '@mui/icons-material/Close';
import * as React from 'react';
import {MouseEvent} from 'react';
import {getTagDisplayText} from '../helper/tagFormatting';
import {secondsToDurationString} from '../helper/secondsToDurationString';

const sliderMaxValueSeconds = 14400;
const sliderStepSize = 60;

const useStyles = makeStyles((theme: Theme) => (
  createStyles({
    root: {
      padding: '2px 4px',
      display: 'flex',
      alignItems: 'center'
    },
    autocomplete: {
      marginLeft: 8,
      flex: 1,
      padding: '4px 0px 3px'
    },
    inputBaseRoot: {
      width: '100%'
    },
    inputBaseInput: {
      padding: '12px 0'
    },
    tagSearchFieldWrapper: {
      display: 'block',
      textAlign: 'center',
      paddingBottom: theme.spacing(0.75)
    },
    advancedSearchWrapper: {
      width: '100%',
      textAlign: 'center'
    },
    accordionExpandIconWrapper: {
      marginLeft: '10px'
    },
    loadingSpinner: {
      marginTop: '12px'
    }
  })
));

const SelectableChipWrapper = styled('div')(({theme}) => ({
  margin: theme.spacing(0.5),
  display: 'inline-flex'
}));

const DeletableChipWrapper = styled('div')(({theme}) => ({
  marginLeft: theme.spacing(0.5),
  marginTop: '3px'
}));

interface SearchBarProps {
  searchText: string
  setSearchText: (query: string) => void
  tagFilter: string
  setTagFilter: (filter: string) => void
  searchTags: string[]
  setSearchTags: (tags: string[]) => void
  tagsWithCounts: {tags: {tag: string, count: number}[], remainingTagCount: number}
  isLoadingTagsWithCounts: boolean
  minLengthSeconds: number | undefined
  setMinLengthSeconds: (seconds: number | undefined) => void
  maxLengthSeconds: number | undefined
  setMaxLengthSeconds: (seconds: number | undefined) => void
}

const SearchBar = (props: SearchBarProps) => {
  const handleMouseDownSearch = (event: MouseEvent) => {
    event.preventDefault();
  };

  const classes = useStyles();

  const getSelectableTagChips = () => {
    const tagChips = props.tagsWithCounts.tags.map(({tag, count}) => (
      <Chip
        onClick={() => props.setSearchTags([...props.searchTags, tag])}
        label={`${getTagDisplayText(tag)} (${count})`}
      />
    ));

    const nonVisibleTagCount = props.tagsWithCounts.remainingTagCount;

    if (nonVisibleTagCount > 0) {
      tagChips.push(<Chip
        label={`... +${nonVisibleTagCount}`}
      />);
    }

    return tagChips.map((chip, index) => <SelectableChipWrapper key={index}>{chip}</SelectableChipWrapper>);
  };

  return (
    <Accordion>
      <AccordionSummary
        expandIcon={<ExpandMoreIcon/>}
        classes={{expandIconWrapper: classes.accordionExpandIconWrapper}}
      >
        <div
          onClick={(event) => event.stopPropagation()}
          onFocus={(event) => event.stopPropagation()}
          style={{
            width: '100%',
            display: 'flex'
          }}
        >
          <Autocomplete
            freeSolo
            options={[]} // TODO - Re-enable autocomplete suggestions by setting some state here.
            className={classes.autocomplete}
            inputValue={props.searchText}
            onInputChange={(event, value, reason) => {
              if (!(value.length === 0 && reason === 'reset')) {
                props.setSearchText(value);
              }
            }}
            renderInput={(params: any) => (
              <div ref={params.InputProps.ref}>
                <InputBase
                  classes={{root: classes.inputBaseRoot, input: classes.inputBaseInput}}
                  placeholder='Search Freedomain Videos'
                  {...params.inputProps}
                />
              </div>
            )}
          />
          {!!props.searchText.length && (
            <IconButton
              sx={{padding: '7px'}}
              onMouseDown={handleMouseDownSearch}
              onClick={() => {
                props.setSearchText('');
              }}
            >
              <CloseIcon/>
            </IconButton>
          )}
          {!!props.searchText.length && <Divider sx={{marginLeft: '4px'}} orientation={'vertical'}/>}
          {!!props.searchTags.length && props.searchTags.map((tag, index) => (
            <DeletableChipWrapper key={index}>
              <Chip
                onDelete={() => props.setSearchTags(props.searchTags.filter((iterTag) => tag !== iterTag))}
                label={getTagDisplayText(tag)}
              />
            </DeletableChipWrapper>
          ))}
        </div>
      </AccordionSummary>
      <Divider/>
      <AccordionDetails sx={{padding: '15px 10px 10px 10px'}}>
        <div className={classes.advancedSearchWrapper}>
          <Slider
            sx={{maxWidth: '210px'}}
            min={0}
            max={sliderMaxValueSeconds}
            step={sliderStepSize}
            valueLabelDisplay={'auto'}
            valueLabelFormat={(value) => ((value === 0 || value === sliderMaxValueSeconds) ? '-' : secondsToDurationString(value))}
            value={[props.minLengthSeconds || 0, props.maxLengthSeconds || sliderMaxValueSeconds]}
            disableSwap
            onChange={
              (
                event,
                newValue: number | number[],
                activeThumb: number
              ) => {
                const minDistance = sliderStepSize * 30;

                if (!Array.isArray(newValue)) {
                  return;
                }
            
                if (activeThumb === 0) {
                  const newVal = Math.min(newValue[0], (props.maxLengthSeconds || sliderMaxValueSeconds) - minDistance);
                  if (newVal <= 0) {
                    props.setMinLengthSeconds(undefined);
                  } else {
                    props.setMinLengthSeconds(newVal);
                  }
                } else {
                  const newVal = Math.max(newValue[1], (props.minLengthSeconds || 0) + minDistance);
                  if (newVal >= sliderMaxValueSeconds) {
                    props.setMaxLengthSeconds(undefined);
                  } else {
                    props.setMaxLengthSeconds(newVal);
                  }
                }
              }
            }
          />
          <Divider sx={{margin: '10px 0'}}/>
          <div className={classes.tagSearchFieldWrapper}>
            <TextField
              value={props.tagFilter}
              onChange={(e) => props.setTagFilter(e.target.value)} label={'Tag Filter'}
            />
          </div>
          {props.isLoadingTagsWithCounts ?
            <CircularProgress className={classes.loadingSpinner}/> : getSelectableTagChips()}
        </div>
      </AccordionDetails>
    </Accordion>
  );
};

export default SearchBar;