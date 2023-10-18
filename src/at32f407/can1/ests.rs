#[doc = "Register `ESTS` reader"]
pub type R = crate::R<ESTS_SPEC>;
#[doc = "Register `ESTS` writer"]
pub type W = crate::W<ESTS_SPEC>;
#[doc = "Field `EAF` reader - Error active flag"]
pub type EAF_R = crate::BitReader<EAF_A>;
#[doc = "Error active flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EAF_A {
    #[doc = "0: Error active state is not entered"]
    NotEntered = 0,
    #[doc = "1: Error active state is entered"]
    Entered = 1,
}
impl From<EAF_A> for bool {
    #[inline(always)]
    fn from(variant: EAF_A) -> Self {
        variant as u8 != 0
    }
}
impl EAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EAF_A {
        match self.bits {
            false => EAF_A::NotEntered,
            true => EAF_A::Entered,
        }
    }
    #[doc = "Error active state is not entered"]
    #[inline(always)]
    pub fn is_not_entered(&self) -> bool {
        *self == EAF_A::NotEntered
    }
    #[doc = "Error active state is entered"]
    #[inline(always)]
    pub fn is_entered(&self) -> bool {
        *self == EAF_A::Entered
    }
}
#[doc = "Field `EPF` reader - Error passive flag"]
pub type EPF_R = crate::BitReader<EPF_A>;
#[doc = "Error passive flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPF_A {
    #[doc = "0: Error passive state is not entered"]
    NotEntered = 0,
    #[doc = "1: Error passive state is entered"]
    Entered = 1,
}
impl From<EPF_A> for bool {
    #[inline(always)]
    fn from(variant: EPF_A) -> Self {
        variant as u8 != 0
    }
}
impl EPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPF_A {
        match self.bits {
            false => EPF_A::NotEntered,
            true => EPF_A::Entered,
        }
    }
    #[doc = "Error passive state is not entered"]
    #[inline(always)]
    pub fn is_not_entered(&self) -> bool {
        *self == EPF_A::NotEntered
    }
    #[doc = "Error passive state is entered"]
    #[inline(always)]
    pub fn is_entered(&self) -> bool {
        *self == EPF_A::Entered
    }
}
#[doc = "Field `BOF` reader - Bus-off flag"]
pub type BOF_R = crate::BitReader<BOF_A>;
#[doc = "Bus-off flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOF_A {
    #[doc = "0: Bus-off state is not entered"]
    NotEntered = 0,
    #[doc = "1: Bus-off state is entered"]
    Entered = 1,
}
impl From<BOF_A> for bool {
    #[inline(always)]
    fn from(variant: BOF_A) -> Self {
        variant as u8 != 0
    }
}
impl BOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOF_A {
        match self.bits {
            false => BOF_A::NotEntered,
            true => BOF_A::Entered,
        }
    }
    #[doc = "Bus-off state is not entered"]
    #[inline(always)]
    pub fn is_not_entered(&self) -> bool {
        *self == BOF_A::NotEntered
    }
    #[doc = "Bus-off state is entered"]
    #[inline(always)]
    pub fn is_entered(&self) -> bool {
        *self == BOF_A::Entered
    }
}
#[doc = "Field `ETR` reader - Error type record"]
pub type ETR_R = crate::FieldReader<ETR_A>;
#[doc = "Error type record\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Bit stuffing error"]
    BitStuffing = 1,
    #[doc = "2: Format error"]
    Format = 2,
    #[doc = "3: Acknowledgement error"]
    Acknowledgement = 3,
    #[doc = "4: Recessive bit error"]
    RecessiveBit = 4,
    #[doc = "5: Dominant bit error"]
    DominantBit = 5,
    #[doc = "6: CRC error"]
    Crc = 6,
    #[doc = "7: Set by software"]
    Software = 7,
}
impl From<ETR_A> for u8 {
    #[inline(always)]
    fn from(variant: ETR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETR_A {
    type Ux = u8;
}
impl ETR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETR_A {
        match self.bits {
            0 => ETR_A::NoError,
            1 => ETR_A::BitStuffing,
            2 => ETR_A::Format,
            3 => ETR_A::Acknowledgement,
            4 => ETR_A::RecessiveBit,
            5 => ETR_A::DominantBit,
            6 => ETR_A::Crc,
            7 => ETR_A::Software,
            _ => unreachable!(),
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ETR_A::NoError
    }
    #[doc = "Bit stuffing error"]
    #[inline(always)]
    pub fn is_bit_stuffing(&self) -> bool {
        *self == ETR_A::BitStuffing
    }
    #[doc = "Format error"]
    #[inline(always)]
    pub fn is_format(&self) -> bool {
        *self == ETR_A::Format
    }
    #[doc = "Acknowledgement error"]
    #[inline(always)]
    pub fn is_acknowledgement(&self) -> bool {
        *self == ETR_A::Acknowledgement
    }
    #[doc = "Recessive bit error"]
    #[inline(always)]
    pub fn is_recessive_bit(&self) -> bool {
        *self == ETR_A::RecessiveBit
    }
    #[doc = "Dominant bit error"]
    #[inline(always)]
    pub fn is_dominant_bit(&self) -> bool {
        *self == ETR_A::DominantBit
    }
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == ETR_A::Crc
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == ETR_A::Software
    }
}
#[doc = "Field `ETR` writer - Error type record"]
pub type ETR_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, ETR_A>;
impl<'a, REG, const O: u8> ETR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::NoError)
    }
    #[doc = "Bit stuffing error"]
    #[inline(always)]
    pub fn bit_stuffing(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::BitStuffing)
    }
    #[doc = "Format error"]
    #[inline(always)]
    pub fn format(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::Format)
    }
    #[doc = "Acknowledgement error"]
    #[inline(always)]
    pub fn acknowledgement(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::Acknowledgement)
    }
    #[doc = "Recessive bit error"]
    #[inline(always)]
    pub fn recessive_bit(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::RecessiveBit)
    }
    #[doc = "Dominant bit error"]
    #[inline(always)]
    pub fn dominant_bit(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::DominantBit)
    }
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::Crc)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_A::Software)
    }
}
#[doc = "Field `TEC` reader - Transmit error counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter"]
pub type REC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Error active flag"]
    #[inline(always)]
    pub fn eaf(&self) -> EAF_R {
        EAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error passive flag"]
    #[inline(always)]
    pub fn epf(&self) -> EPF_R {
        EPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off flag"]
    #[inline(always)]
    pub fn bof(&self) -> BOF_R {
        BOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Error type record"]
    #[inline(always)]
    pub fn etr(&self) -> ETR_R {
        ETR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Transmit error counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive error counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESTS")
            .field("rec", &format_args!("{}", self.rec().bits()))
            .field("tec", &format_args!("{}", self.tec().bits()))
            .field("etr", &format_args!("{}", self.etr().bits()))
            .field("bof", &format_args!("{}", self.bof().bit()))
            .field("epf", &format_args!("{}", self.epf().bit()))
            .field("eaf", &format_args!("{}", self.eaf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ESTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error type record"]
    #[inline(always)]
    #[must_use]
    pub fn etr(&mut self) -> ETR_W<ESTS_SPEC, 4> {
        ETR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ests::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESTS_SPEC;
impl crate::RegisterSpec for ESTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ests::R`](R) reader structure"]
impl crate::Readable for ESTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ests::W`](W) writer structure"]
impl crate::Writable for ESTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESTS to value 0"]
impl crate::Resettable for ESTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
